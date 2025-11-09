import type { ErrorModule } from "./errors";

type LogLevel = "debug" | "info" | "warn" | "error";

interface LogEntry {
  level: LogLevel;
  message: string;
  timestamp: string;
  module?: ErrorModule;
  context?: Record<string, unknown>;
  error?: {
    message: string;
    stack?: string;
  };
}

class Logger {
  private isDevelopment = process.env.NODE_ENV === "development";
  private logBuffer: LogEntry[] = [];
  private readonly MAX_BUFFER_SIZE = 100;

  private formatMessage(entry: LogEntry): string {
    const { level, message, timestamp, context, error, module } = entry;
    const moduleStr = module ? ` [${module}]` : "";
    const contextStr = context ? ` ${JSON.stringify(context)}` : "";
    const errorStr = error
      ? ` Error: ${error.message}${error.stack ? `\n${error.stack}` : ""}`
      : "";
    return `[${timestamp}] [${level.toUpperCase()}]${moduleStr} ${message}${contextStr}${errorStr}`;
  }

  private addToBuffer(entry: LogEntry) {
    this.logBuffer.push(entry);
    if (this.logBuffer.length > this.MAX_BUFFER_SIZE) {
      this.logBuffer.shift();
    }
  }

  private log(
    level: LogLevel,
    message: string,
    context?: Record<string, unknown>,
    error?: Error,
    module?: ErrorModule
  ) {
    const entry: LogEntry = {
      level,
      message,
      timestamp: new Date().toISOString(),
      module,
      context,
      error: error
        ? {
            message: error.message,
            stack: error.stack,
          }
        : undefined,
    };

    this.addToBuffer(entry);

    if (this.isDevelopment) {
      const formatted = this.formatMessage(entry);

      if (typeof window !== "undefined" && window.dispatchEvent) {
        window.dispatchEvent(
          new CustomEvent("log", {
            detail: { level, message: formatted, entry },
          })
        );
      }

      if (typeof process !== "undefined" && process.stdout) {
        const output = level === "error" ? process.stderr : process.stdout;
        output.write(`${formatted}\n`);
      }
    }

    if (level === "error" && !this.isDevelopment) {
      this.sendToErrorService(entry);
    }
  }

  private sendToErrorService(entry: LogEntry) {
    if (typeof window !== "undefined" && typeof fetch !== "undefined") {
      fetch("/api/log", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(entry),
      }).catch(() => {
        // Silently fail if error logging service is unavailable
      });
    }
  }

  debug(message: string, context?: Record<string, unknown>, module?: ErrorModule) {
    this.log("debug", message, context, undefined, module);
  }

  info(message: string, context?: Record<string, unknown>, module?: ErrorModule) {
    this.log("info", message, context, undefined, module);
  }

  warn(message: string, context?: Record<string, unknown>, module?: ErrorModule) {
    this.log("warn", message, context, undefined, module);
  }

  error(message: string, error?: Error, context?: Record<string, unknown>, module?: ErrorModule) {
    this.log("error", message, context, error, module);
  }

  getLogs(): readonly LogEntry[] {
    return this.logBuffer;
  }

  clearLogs() {
    this.logBuffer = [];
  }
}

export const logger = new Logger();
