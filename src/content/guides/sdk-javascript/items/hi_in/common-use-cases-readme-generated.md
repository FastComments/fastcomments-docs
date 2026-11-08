### किसी पेज के लिए कमेंट्स प्राप्त करना

```typescript
const comments = await sdk.publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'article-123'
});
```

### एक कमेंट बनाना

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

### कमेंट पर वोट करना

```typescript
const voteResponse = await sdk.publicApi.voteComment({
  voteBodyParams: {
    commentId: 'comment-id',
    direction: 1 // अपवोट के लिए 1, डाउनवोट के लिए -1
  }
});
```

### यूज़र प्रबंधन (API Key आवश्यक है)

```typescript
// यूज़र खोजें (DefaultApi आवश्यक है)
const users = await sdk.defaultApi.searchUsers({
  tenantId: 'your-tenant-id',
  urlId: 'page-id',
  usernameStartsWith: 'john'
});
```