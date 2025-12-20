use sqlx::{MySql, Pool};

pub async fn run_migrations(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    // users table
    sqlx::query(
        r#"
create table if not exists users (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(255) NOT NULL unique,
    display_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role ENUM('admin', 'author', 'reader') NOT NULL DEFAULT 'reader',
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    INDEX idx_email (email)
)   ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
    "#,
    )
    .execute(pool)
    .await?;

    // categories table
    sqlx::query(
        r#"
create table if not exists categories (
    id int unsigned primary key auto_increment,
    name varchar(255) not null unique,
    slug varchar(255) not null unique,
    description text null,
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp on update current_timestamp,

    index idx_slug (slug)
)   engine=Innodb default charset=utf8mb4 collate=utf8mb4_unicode_ci;
    "#,
    )
    .execute(pool)
    .await?;

    // posts table
    sqlx::query(
        r#"
create table if not exists posts (
    id int unsigned primary key auto_increment,
    title varchar(255) not null,
    slug varchar(255) not null unique,
    author_id int unsigned not null,
    category_id int unsigned,
    excerpt text,
    content text not null,
    status enum('draft', 'published', 'archived') not null default 'draft',
    published_at timestamp null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp on update current_timestamp,

    foreign key (author_id) references users(id) on delete cascade,
    foreign key (category_id) references categories(id) on delete set null,

    index idx_slug (slug),
    index idx_author (author_id),
    index idx_status (status),
    index idx_published (published_at)
)   engine=InnoDB default charset=utf8mb4 collate=utf8mb4_unicode_ci;
      "#,
    )
    .execute(pool)
    .await?;

    // Post likes
    sqlx::query(
        r#"
create table if not exists post_likes (
    post_id int unsigned not null,
    user_id int unsigned not null,
    created_at timestamp default current_timestamp not null,

    primary key (post_id, user_id),

    foreign key (post_id) references posts(id) on delete cascade,
    foreign key (user_id) references users(id) on delete cascade
)   engine=InnoDB;
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
create table if not exists tags (
    id int unsigned primary key auto_increment,
    name varchar(100) not null unique,
    slug varchar(100) not null unique,
    created_at timestamp default current_timestamp
);
    "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
create table if not exists post_tags (
    post_id int unsigned not null,
    tag_id int unsigned not null,
    created_at timestamp default current_timestamp,

    primary key (post_id, tag_id),

    foreign key (post_id) references posts(id) on delete cascade,
    foreign key (tag_id) references tags(id) on delete cascade
);
    "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
