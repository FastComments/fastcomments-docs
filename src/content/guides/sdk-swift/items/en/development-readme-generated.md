### Regenerating the Client

To regenerate the API client from the latest OpenAPI specification:

1. Ensure you have the FastComments server running locally at `http://localhost:3001`
2. Run the update script:

```bash
./update.sh
```

This will:
- Download the latest OpenAPI spec
- Generate the Swift client code (with API documentation in client/docs)
- Build the package to verify everything works