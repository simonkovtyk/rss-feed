use sqlx::{Pool, Postgres};

pub async fn init_db (pool: &Pool<Postgres>) {
  sqlx::query("
    CREATE TABLE IF NOT EXISTS public.channels (
      name text NULL,
      checksum varchar NOT NULL,
      id bigserial NOT NULL,
      image_url text NULL,
      CONSTRAINT channels_pk PRIMARY KEY (id)
    );
  ")
    .fetch_optional(pool)
    .await
    .expect("Could not init channels table");

  sqlx::query("
    CREATE TABLE IF NOT EXISTS public.posts (
      id bigserial NOT NULL,
      title text NULL,
      pub_date text NULL,
      content text NULL,
      checksum varchar NOT NULL,
      link text NULL,
      channel_id bigserial NOT NULL,
      CONSTRAINT posts_pk PRIMARY KEY (id),
      CONSTRAINT posts_channel_fk FOREIGN KEY (channel_id)
        REFERENCES public.channels (id)
    );
  ")
    .fetch_optional(pool)
    .await
    .expect("Could not init posts table");
}