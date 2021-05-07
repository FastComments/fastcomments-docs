In the event that data needs to be moved around, FastComments provides a self-service tool for moving comments
between pages and articles.

Here's what the comment copy page form looks like:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Filling out the "From" Fields

To decide where to move comments from, we simply need to know the source `URL ID`.

If you aren't passing a value for `urlId` in the comment widget configuration, then this will be a "clean" version of the page URL.

You can see what values your comments have for `URL ID` by exporting them.

### Filling out the "To" Fields

To decide where to move comments to, we need to know the target `URL ID` and `URL`.

The `URL ID` will be the bucket that the comment goes in. The `URL` field is used so that you can navigate directly
to the comment from emails and moderation tools.

#### WordPress

If you are using WordPress, you would for example enter the Article IDs in the To/From `URL ID` fields in the migration tool,
rather than a URL.
