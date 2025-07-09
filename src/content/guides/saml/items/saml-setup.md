Setting up SAML authentication in FastComments requires both configuration in your admin dashboard and setup in your identity provider.

### Prerequisites

Before configuring SAML, ensure you have:

- A FastComments Flex or Pro plan (SAML is not available on the Creators plan)
- Administrative access to your FastComments account
- Administrative access to your identity provider
- Your IdP's SAML metadata or certificate information

### Accessing SAML Configuration

1. Log into your [FastComments admin dashboard](https://fastcomments.com/auth/my-account)
2. Navigate to **API/SSO Settings** in the left sidebar
3. Click the **SAML Config** button

If you don't see the SAML Config button, verify that:
- Your account has the required package (Flex or Pro)
- You have administrative permissions
- Your user has API Admin or Admin Admin roles

### Basic SAML Configuration

#### Enable SAML Authentication

1. Check the **Enable SAML Authentication** checkbox
2. This activates SAML for your tenant and makes the configuration fields available

#### Required Fields

**IdP Single Sign-On URL** *(Required)*
- The URL where users will be redirected for authentication
- Usually provided by your identity provider
- Example: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Required)*
- The public certificate from your identity provider
- Used to verify the authenticity of SAML responses
- Must include the full certificate with BEGIN/END markers
- Example format:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### Optional Fields

**IdP Entity ID / Issuer**
- Identifies your identity provider
- If left blank, defaults to your FastComments URL
- Should match the issuer configured in your IdP

### Advanced Configuration

#### Security Settings

**Signature Algorithm**
- Defaults to SHA-256 (recommended)
- Options: SHA-1, SHA-256, SHA-512
- Should match your IdP's configuration

**Digest Algorithm**
- Defaults to SHA-256 (recommended)
- Used for digest computation in SAML responses
- Should match your IdP's configuration

**Name ID Format**
- Defaults to Email Address format
- Determines how user identifiers are formatted
- Common options: Email Address, Persistent, Transient

#### Encryption (Optional)

**Private Key for Decryption**
- Only needed if your IdP encrypts SAML assertions
- Paste your private key used for decryption
- Most deployments don't require assertion encryption

### Saving Configuration

1. Review all settings for accuracy
2. Click **Save SAML Configuration**
3. The system will validate your configuration
4. If successful, you'll see a confirmation message

### Next Steps

After saving your FastComments SAML configuration:

1. Configure your identity provider using the Service Provider information
2. Test the authentication flow
3. Set up user roles and permissions as needed

The Service Provider information needed for your IdP configuration will be displayed once SAML is enabled.
