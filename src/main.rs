use clap::{Parser, Subcommand};
use std::fs;
use std::io::Write;
use std::fs::read_to_string;
use serde::Deserialize;
use toml;

// Import our custom modules
mod markdown;
use markdown::{parse_page_markdown, parse_post_markdown, Post};

// Import the server module
mod server;
use server::start_server;

/// Struct to hold site configuration loaded from `config.toml`
#[derive(Deserialize)]
struct SiteConfig {
    site: SiteInfo,
    links: Links,
}

/// Holds site metadata like title, description, etc.
#[derive(Deserialize)]
struct SiteInfo {
    title: String,
    description: String,
    author: String,
    profile_picture: String,
}

/// Holds external links
#[derive(Deserialize)]
struct Links {
    github: String,
    twitter: String,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site (parse Markdown & generate HTML)
    Build,
    /// Start a local server to preview the site at http://localhost:8464
    Serve,
}

fn main() {
    let cli = Cli::parse();

    // Load the configuration from `config.toml`
    let config = load_config();

    match cli.command {
        Commands::Build => {
            println!("Building site...");

            // Ensure `docs/posts` folder exists
            fs::create_dir_all("docs/posts")
                .expect("Failed to create or verify docs/posts directory");

            // Collect blog posts to build index.html
            let mut posts_collected: Vec<Post> = Vec::new();

            // Scan `content/` for .md files
            if let Ok(entries) = fs::read_dir("content") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("md") {
                        let file_path = path.to_string_lossy().to_string();

                        // Check special pages
                        if file_path.ends_with("about.md") {
                            generate_about(&file_path, &config);
                        } else if file_path.ends_with("license.md") {
                            generate_license(&file_path, &config);
                        } else {
                            // Treat everything else as a blog post
                            match parse_post_markdown(&file_path) {
                                Ok(post) => {
                                    // Build the final HTML for this post using wrap_in_template
                                    // We'll pass in the post's title and a custom body content.
                                    let post_body = format!(
                                        "<h1>{title}</h1>
                                         <p><strong>By {author}</strong> - {date} - {read_time} min read</p>
                                         {content}",
                                        title = post.front_matter.title,
                                        author = post.front_matter.author,
                                        date = post.front_matter.date,
                                        read_time = post.reading_time,
                                        content = post.content,
                                    );

                                    let final_html = wrap_in_template(
                                        &post.front_matter.title,
                                        &post_body,
                                        "../",
                                        &config
                                    );

                                    // Write it out to post.file_name
                                    let mut file = fs::File::create(&post.file_name)
                                        .expect("Failed to create post file");
                                    file.write_all(final_html.as_bytes())
                                        .expect("Failed to write post file");

                                    println!("Generated: {}", post.file_name);

                                    // Add to list for index.html
                                    posts_collected.push(post);
                                }
                                Err(e) => {
                                    println!("Error parsing post {}: {}", file_path, e);
                                }
                            }
                        }
                    }
                }
            }

            // Generate index.html to link to all posts
            generate_index(&posts_collected, &config);

            generate_posts(&posts_collected, &config);

            println!("Site build complete!");
        }

        Commands::Serve => {
            // Start server on a custom port
            println!("Starting server at http://localhost:8464...");
            let port = 8464;
            if let Err(e) = start_server(port) {
                eprintln!("Server error: {}", e);
            }
        }
    }
}

/// Load configuration from `config.toml`
fn load_config() -> SiteConfig {
    let config_contents =
        read_to_string("config.toml").expect("Failed to read config.toml");
    toml::from_str(&config_contents).expect("Failed to parse config.toml")
}

/// Helper function to generate the full HTML layout
/// Takes a `title` (for <title>) and a `body_content` (the unique page/post body).
fn wrap_in_template(title: &str, body_content: &str, prefix: &str, config: &SiteConfig) -> String {
    format!(
"<!DOCTYPE html>
<html lang='en'>
<head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <title>{title}</title>
    <link rel='stylesheet' href='https://unpkg.com/@picocss/pico@latest/css/pico.min.css'>
    <link rel='stylesheet' href='{prefix}assets/styles.css'>
</head>
<body>
<header class='container'>
  <nav class='flex-nav'>
    <!-- Left side: brand or home icon -->
    <ul class='nav-left'>
      <li><a href='{prefix}index.html'><i data-feather='home'></i></a></li>
    </ul>

    <!-- Right side: links -->
    <ul class='nav-links' id='navLinks'>
      <li><a href='{prefix}index.html'>Home</a></li>
      <li><a href='{prefix}posts.html'>Posts</a></li>
      <li><a href='{prefix}about.html'>About</a></li>
      <li><a href='{prefix}license.html'>License</a></li>
    </ul>

    <!-- Hamburger Button (hidden on large screens) -->
    <button
      class='hamburger'
      aria-label='Toggle Menu'
      onclick='toggleMenu()'>
      <i data-feather='menu'></i>
    </button>
  </nav>
</header>
<main class='container'>
{body_content}
</main>
<footer class='container' style='text-align: center;'>
    <p>
        <a href='{github}'><i data-feather='github'></i></a>
        &nbsp;|&nbsp;
        <a href='{twitter}'><i data-feather='twitter'></i></a>
    </p>
    <p class='footer-credit'>
        © 2025 <a href='{twitter}'>{author}</a>. Powered by <a href='https://github.com/0xh4ty/xeniria'>Xeniria</a>.
    </p>
</footer>
<script src='https://unpkg.com/feather-icons'></script>
<script>feather.replace();</script>
<script>
  function toggleMenu() {{
    let nav = document.getElementById('navLinks');
    nav.classList.toggle('open');
  }}
</script>
</body>
</html>",
        title = title,
        prefix = prefix,
        body_content = body_content,
        author = config.site.author,
        github = config.links.github,
        twitter = config.links.twitter
    )
}

/// Generate `about.html` from `about.md`
fn generate_about(file_path: &str, config: &SiteConfig) {
    match parse_page_markdown(file_path) {
        Ok(page) => {
            // Prepare a body with a heading, author, and page.content
            let about_body = format!(
                "<h1>{title}</h1>
                 <p>By {author}</p>
                 {content}",
                title = page.front_matter.title,
                author = page.front_matter.author,
                content = page.content
            );

            let final_html = wrap_in_template(&page.front_matter.title, &about_body, "", config);

            let mut file = fs::File::create("docs/about.html")
                .expect("Failed to create about.html");
            file.write_all(final_html.as_bytes())
                .expect("Failed to write about.html");

            println!("Generated: docs/about.html");
        }
        Err(e) => {
            println!("Error parsing About page {}: {}", file_path, e);
        }
    }
}

/// Generate `license.html` from `license.md`
fn generate_license(file_path: &str, config: &SiteConfig) {
    match parse_page_markdown(file_path) {
        Ok(page) => {
            let license_body = format!(
                "<h1>{title}</h1>
                 <p>By {author}</p>
                 {content}",
                title = page.front_matter.title,
                author = page.front_matter.author,
                content = page.content
            );

            let final_html = wrap_in_template(&page.front_matter.title, &license_body, "", config);

            let mut file = fs::File::create("docs/license.html")
                .expect("Failed to create license.html");
            file.write_all(final_html.as_bytes())
                .expect("Failed to write license.html");

            println!("Generated: docs/license.html");
        }
        Err(e) => {
            println!("Error parsing License page {}: {}", file_path, e);
        }
    }
}

/// Generate `index.html` listing all blog posts
fn generate_index(posts: &Vec<Post>, config: &SiteConfig) {
    // Clone & sort posts by date DESC (assuming YYYY-MM-DD format)
    let mut sorted_posts = posts.clone();
    sorted_posts.sort_by(|a, b| b.front_matter.date.cmp(&a.front_matter.date));

    // Build the "Recent Posts" list
    let mut recent_posts_html = format!(
        "<div class='profile-container'>
            <img class='profile-img' src='{profile_picture}' alt='Profile Picture'>
            <h2 class='profile-name'>{author}</h2>
            <p class='profile-desc'>{description}</p>
        </div>
        <div class='recent-posts'>
        <h3>Recent Posts</h3>
        <ul>",
        profile_picture = config.site.profile_picture,
        author = config.site.author,
        description = config.site.description,
    );

    for post in sorted_posts.iter().take(5) {
        recent_posts_html.push_str("<hr>\n");
        let link_path = post.file_name.replace("docs/", "");
        recent_posts_html.push_str(&format!(
            "<li class='post-item'>
                <span class='post-title'>
                    <a href='{link}'>{title}</a>
                </span>
                <span class='post-date'>
                    {date}
                </span>
            </li>\n",
            link = link_path,
            title = post.front_matter.title,
            date = post.front_matter.date
        ));
    }

    recent_posts_html.push_str(
        "</ul>
        <div style='text-align:center;'>
          <a href='posts.html'>See all posts</a>
        </div>
        </div>"
    );

    // Wrap the "recent_posts_html" in the global template
    let final_html = wrap_in_template(
        &config.site.title,
        &recent_posts_html,
        "",
        config
    );

    // Write to `docs/index.html`
    let mut file = fs::File::create("docs/index.html")
        .expect("Failed to create docs/index.html");
    file.write_all(final_html.as_bytes())
        .expect("Failed to write index.html");

    println!("Generated: docs/index.html");
}

/// Generate `posts.html` listing all posts grouped by year
fn generate_posts(posts: &Vec<Post>, config: &SiteConfig) {
    // Clone & sort posts by date DESC (newest first)
    let mut sorted_posts = posts.clone();
    sorted_posts.sort_by(|a, b| b.front_matter.date.cmp(&a.front_matter.date));

    // Start HTML content
    let mut posts_html = String::from("<div class='posts-container'>\n<h1 style='text-align: center;'>Posts</h1>\n");

    let mut last_year = String::new();

    for post in &sorted_posts {
        let post_year = &post.front_matter.date[..4]; // Extract YYYY from YYYY-MM-DD

        // If it's a new year, add a heading with extra spacing
        if post_year != last_year {
            posts_html.push_str(&format!("<h3 class='post-year'>{}</h3>\n", post_year));
            last_year = post_year.to_string();
        }

        let link_path = post.file_name.replace("docs/", "");
        posts_html.push_str(&format!(
            "<hr>\n\
             <div class='post-item'>\n\
                <a href='{link}' class='post-title'>{title}</a>\n\
                <span class='post-date'>{date}</span>\n\
            </div>\n",
            link = link_path,
            title = post.front_matter.title,
            date = post.front_matter.date
        ));
    }

    // Close the `posts-container` div
    posts_html.push_str("</div>\n");

    // Wrap in template
    let final_html = wrap_in_template("All Posts", &posts_html, "", config);

    // Write to `docs/posts.html`
    let mut file = fs::File::create("docs/posts.html")
        .expect("Failed to create docs/posts.html");
    file.write_all(final_html.as_bytes())
        .expect("Failed to write posts.html");

    println!("Generated: docs/posts.html");
}
