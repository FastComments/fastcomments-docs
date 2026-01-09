### Dobijanje komentara za stranicu

```typescript
const comments = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'article-123'
});
```

### Kreiranje komentara

```typescript
const newComment = await sdk.publicApi.createCommentPublic({
  createCommentParams: {
    tenantId: 'your-tenant-id',
    urlId: 'article-123',
    comment: 'Great article!',
    commenterName: 'John Doe',
    commenterEmail: 'john@example.com'
  }
});
```

### Glasanje za komentar

```typescript
const voteResponse = await sdk.publicApi.voteComment({
  voteBodyParams: {
    commentId: 'comment-id',
    direction: 1 // 1 za podršku, -1 za protiv
  }
});
```

### Upravljanje korisnicima (zahteva API ključ)

```typescript
// Pretraži korisnike (zahteva DefaultApi)
const users = await sdk.defaultApi.searchUsers({
  tenantId: 'your-tenant-id',
  urlId: 'page-id',
  usernameStartsWith: 'john'
});
```