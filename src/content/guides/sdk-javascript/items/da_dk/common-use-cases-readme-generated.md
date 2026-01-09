### Hent kommentarer for en side

```typescript
const comments = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'article-123'
});
```

### Oprettelse af en kommentar

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

### Stem på en kommentar

```typescript
const voteResponse = await sdk.publicApi.voteComment({
  voteBodyParams: {
    commentId: 'comment-id',
    direction: 1 // 1 for opstemning, -1 for nedstemning
  }
});
```

### Brugeradministration (Kræver API-nøgle)

```typescript
// Søg efter brugere (kræver DefaultApi)
const users = await sdk.defaultApi.searchUsers({
  tenantId: 'your-tenant-id',
  urlId: 'page-id',
  usernameStartsWith: 'john'
});
```