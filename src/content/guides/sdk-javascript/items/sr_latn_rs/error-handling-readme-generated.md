```typescript
try {
  const comments = await sdk.publicApi.getCommentsPublic({
    tenantId: 'your-tenant-id',
    urlId: 'page-id'
  });
} catch (error) {
  if (error.response?.status === 404) {
    console.log('Page not found');
  } else {
    console.error('API Error:', error.message);
  }
}
```