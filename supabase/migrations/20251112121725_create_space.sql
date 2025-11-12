
  create table "public"."space" (
    "slug" character varying(30) not null,
    "created_at" timestamp with time zone default now(),
    "created_by" uuid not null
      );


CREATE UNIQUE INDEX space_pkey ON public.space USING btree (slug);

alter table "public"."space" add constraint "space_pkey" PRIMARY KEY using index "space_pkey";

alter table "public"."space" add constraint "space_created_by_fkey" FOREIGN KEY (created_by) REFERENCES auth.users(id) ON DELETE CASCADE not valid;

alter table "public"."space" validate constraint "space_created_by_fkey";

grant delete on table "public"."space" to "anon";

grant insert on table "public"."space" to "anon";

grant references on table "public"."space" to "anon";

grant select on table "public"."space" to "anon";

grant trigger on table "public"."space" to "anon";

grant truncate on table "public"."space" to "anon";

grant update on table "public"."space" to "anon";

grant delete on table "public"."space" to "authenticated";

grant insert on table "public"."space" to "authenticated";

grant references on table "public"."space" to "authenticated";

grant select on table "public"."space" to "authenticated";

grant trigger on table "public"."space" to "authenticated";

grant truncate on table "public"."space" to "authenticated";

grant update on table "public"."space" to "authenticated";

grant delete on table "public"."space" to "service_role";

grant insert on table "public"."space" to "service_role";

grant references on table "public"."space" to "service_role";

grant select on table "public"."space" to "service_role";

grant trigger on table "public"."space" to "service_role";

grant truncate on table "public"."space" to "service_role";

grant update on table "public"."space" to "service_role";


