# sqlctx-extension

Enclosed is a [Zed](https://zed.dev/) extension for [sqlctx](https://github.com/nicosuave/sqlctx). It makes it effortless to draw SQL database table definitions and sample data into context in Zed editor's AI panel.

## Usage

_You will need to generate context for each Zed project with the above cli before using this extension._

Please see the [sqlctx](https://github.com/nicosuave/sqlctx) repo for more information on configuration & generating context from databases. 

After installing the `sqlctx` extension from within Zed, you will have the additional slash command available:

- `/sqlctx` - include pre-generated `sqlctx` for a given table name
