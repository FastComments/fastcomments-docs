The script for each extension is fetched and invoked before the comment widget begins fetching the first set of comments and rendering the UI.

On initial load, the following data will be attached to the extension object:

- `config` - A reference to the `config` object.
- `translations` - A reference to the `translations` object.
- `commentsById` - A reference to all comments by id.
- `root` - A reference to the root DOM node.

Extensions should override the desired functions; the comment widget will invoke them at the appropriate times.