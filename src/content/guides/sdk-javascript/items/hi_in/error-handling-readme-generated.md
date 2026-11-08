```typescript
try {
  const comments = await sdk.publicApi.getCommentsPublic({
    tenantId: 'your-tenant-id',
    urlId: 'page-id'
  });
} catch (error) {
  if (error.response?.status === 404) {
    console.log('पेज नहीं मिला');
  } else {
    console.error('API त्रुटि:', error.message);
  }
}
```