# Soup.rs

Soup acts as a public repository of bowl templates, although you
can make some templates private. If you need templates to be
private, you can also host your own repository!

## Web Interface

- Homepage has some links to various pieces of documentation and
the templates page.
- Templates page shows a paginated listing of packages sorted by most
recently published.
- Bar at the top of screen with 
- Automatically generated documentation for subcommands and
configuration options (`bowl doc --open` to preview).

## API

- Have routes to get info about a template
- Have routes to return paginated template lists sorted by most
recently published

## User Ideas

- Require secret token to publish changes.
- Allow user to login through GitHub, GitLab, etc.
