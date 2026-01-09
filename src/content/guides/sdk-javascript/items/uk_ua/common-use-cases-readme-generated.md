### Отримання коментарів для сторінки

```typescript
const comments = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'article-123'
});
```

### Створення коментаря

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

### Голосування за коментар

```typescript
const voteResponse = await sdk.publicApi.voteComment({
  voteBodyParams: {
    commentId: 'comment-id',
    direction: 1 // 1 — за, -1 — проти
  }
});
```

### Управління користувачами (потребує API-ключа)

```typescript
// Пошук користувачів (потребує DefaultApi)
const users = await sdk.defaultApi.searchUsers({
  tenantId: 'your-tenant-id',
  urlId: 'page-id',
  usernameStartsWith: 'john'
});
```