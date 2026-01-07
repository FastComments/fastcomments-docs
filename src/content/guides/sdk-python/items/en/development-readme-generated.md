### Running Tests

```bash
# Set up environment variables
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Run tests
pytest tests/
```

### Regenerating the Client

To regenerate the API client from the latest OpenAPI specification:

```bash
./update.sh
```

This will:
1. Download the latest OpenAPI spec from a running FastComments server (or use local openapi.yaml)
2. Generate the Python client code
3. Flatten the directory structure
4. Clean up redundant configuration files