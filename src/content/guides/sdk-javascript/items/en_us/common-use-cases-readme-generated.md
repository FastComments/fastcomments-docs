### Getting Comments for a Page

```typescript
const comments = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'article-123'
});
```

### Creating a Comment

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

### Voting on a Comment

```typescript
const voteResponse = await sdk.publicApi.voteComment({
  voteBodyParams: {
    commentId: 'comment-id',
    direction: 1 // 1 for upvote, -1 for downvote
  }
});
```

### User Management (Requires API Key)

```typescript
// Search for users (requires DefaultApi)
const users = await sdk.defaultApi.searchUsers({
  tenantId: 'your-tenant-id',
  urlId: 'page-id',
  usernameStartsWith: 'john'
});
```