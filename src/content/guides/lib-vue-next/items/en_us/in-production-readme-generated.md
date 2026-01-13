---
You probably don't want to define the config inline if you're passing callbacks etc. Instead, you'll want to define
the config via `computed()`, otherwise each time your callback etc is invoked the entire widget will re-render.
---