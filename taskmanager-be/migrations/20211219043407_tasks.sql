-- Add migration script here

CREATE TABLE IF NOT EXISTS projects (
    "uuid" character varying(50) NOT NULL
    ,"name" text NOT NULL
    ,PRIMARY KEY ("uuid")
);

CREATE TABLE IF NOT EXISTS tags (
    "uuid" character varying(50) NOT NULL
    ,"label" text NOT NULL
    ,PRIMARY KEY ("uuid")
);

CREATE TABLE IF NOT EXISTS tasks (
    "uuid" character varying(50) NOT NULL
    ,"project_uuid" character varying(50) NOT NULL
    ,"title" text NOT NULL
    ,"category" character(30) NOT NULL
    ,"content" text NOT NULL
    ,"created_at" timestamp with time zone NOT NULL
    ,"deladline" timestamp with time zone
    ,PRIMARY KEY ("uuid")
    ,FOREIGN KEY ("project_uuid") REFERENCES "projects" ("uuid")
);

CREATE TABLE IF NOT EXISTS tag_task (
    "tag_uuid" character varying(50) NOT NULL
    ,"task_uuid" character varying(50) NOT NULL
    ,"created_at" timestamp with time zone NOT NULL
    ,PRIMARY KEY ("tag_uuid", "task_uuid")
    ,FOREIGN KEY ("tag_uuid") REFERENCES "tags" ("uuid")
    ,FOREIGN KEY ("task_uuid") REFERENCES "tasks" ("uuid")
);
