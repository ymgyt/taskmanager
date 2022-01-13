CREATE TABLE IF NOT EXISTS tasks (
    "id" text NOT NULL
    ,"title" text NOT NULL
    ,"category" text NOT NULL
    ,"content" text NOT NULL
    ,"status" text NOT NULL
    ,"created_at" timestamp with time zone NOT NULL
    ,"deadline" timestamp with time zone
    ,PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS  task_dependencies (
--  src --> dependent
    "src" text NOT NULL
    ,"dependent" text NOT NULL
    ,PRIMARY KEY ("src", "dependent")
    ,FOREIGN KEY ("src") REFERENCES tasks ("id")
    ,FOREIGN KEY ("dependent") REFERENCES tasks ("id")
)
