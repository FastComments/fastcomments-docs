Veri taşınması gerektiğinde, FastComments sayfalar ve makaleler arasında yorumları taşımak için kendi kendine hizmet aracı sağlar.

İşte yorum kopyalama sayfası formunun görünümü:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Filling out the "From" Fields

Yorumların nereden taşınacağına karar vermek için, sadece kaynak `URL ID`'sini bilmamız yeterlidir.

If you aren't passing a value for `urlId` in the comment widget configuration, then this will be a "clean" version of the page URL.

### Filling out the "To" Fields

Yorumların nereye taşınacağını belirlemek için hedef `URL ID` ve `URL`'yi bilmemiz gerekir.

The `URL ID` will be the bucket that the comment goes in. The `URL` field is used so that you can navigate directly
to the comment from emails and moderation tools.

#### WordPress

If you are using WordPress, you would for example enter the Article IDs in the To/From `URL ID` fields in the migration tool,
rather than a URL.