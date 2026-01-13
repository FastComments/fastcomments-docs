When SAML is enabled in FastComments, the system automatically generates Service Provider (SP) information that you need to configure in your identity provider.

### Accessing Service Provider Information

The SP information is displayed in your SAML configuration page after enabling SAML authentication. This information includes all the details your identity provider needs to establish the SAML trust relationship.

### Service Provider Endpoints

#### SP Entity ID / Audience
**Purpose**: Uniquely identifies your FastComments instance as a service provider
**Format**: `https://fastcomments.com/saml/{your-tenant-id}`
**Usage**: Configure this as the Entity ID or Audience in your IdP

This identifier ensures that SAML responses are intended for your specific FastComments tenant and prevents SAML responses from being accepted by other instances.

#### Assertion Consumer Service (ACS) URL
**Purpose**: The endpoint where your IdP sends SAML responses after user authentication
**Format**: `https://fastcomments.com/saml/callback/{your-tenant-id}`
**Usage**: Configure this as the ACS URL or Reply URL in your IdP

This is where users are redirected after successful authentication with your identity provider, along with the SAML assertion containing user information.

#### SP Metadata URL
**Purpose**: Provides complete SAML configuration in standard XML format
**Format**: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
**Usage**: Some IdPs can automatically import configuration using this URL

The metadata URL contains all necessary SP information in XML format, making it easy to configure compatible identity providers automatically.

#### SAML Login URL
**Purpose**: Direct link to initiate SAML authentication for your tenant
**Format**: `https://fastcomments.com/saml/login/{your-tenant-id}`
**Usage**: Link users directly to SAML authentication or test the flow

You can use this URL to test SAML authentication or provide users with a direct link to sign in via SAML.

### SAML Binding Support

FastComments supports the following SAML bindings:

#### HTTP-POST Binding
- **Primary Method**: Most common binding for SAML responses
- **Security**: SAML response is sent via HTTP POST to the ACS URL
- **Usage**: Recommended for production deployments

#### HTTP-Redirect Binding
- **Alternative Method**: SAML response sent via HTTP redirect
- **Limitations**: Limited payload size due to URL length restrictions
- **Usage**: Supported but HTTP-POST is preferred

### Name ID Policy

FastComments configures the following Name ID policy in SAML requests:

- **Default Format**: `urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress`
- **Alternative Formats**: Persistent, Transient, Unspecified (configurable)
- **Requirement**: The email address is used as the primary user identifier

### SAML Request Attributes

When initiating SAML authentication, FastComments sends requests with these characteristics:

#### Request Signing
- **Status**: Optional (configurable)
- **Algorithm**: Matches configured signature algorithm
- **Certificate**: Uses tenant-specific certificate if request signing is enabled

#### Requested Attributes
FastComments requests the following attributes in SAML AuthnRequests:

- **Email**: Required for user identification
- **First Name**: Optional for display purposes  
- **Last Name**: Optional for display purposes
- **Roles/Groups**: Optional for access control and permissions

### Copying SP Information

The SAML configuration page provides clickable fields that automatically copy SP information to your clipboard:

1. Click any SP information field (Entity ID, ACS URL, etc.)
2. The value is automatically copied to your clipboard
3. Paste the value into your identity provider configuration
4. A brief highlight indicates successful copying

This makes it easy to accurately transfer the SP information to your IdP without typing errors.

### SP Certificate Information

#### Certificate Usage
- **Purpose**: Encrypts communications and verifies SP identity
- **Rotation**: Certificates are automatically managed by FastComments
- **Access**: Public certificates are available via the metadata URL

#### Certificate Details
- **Algorithm**: RSA-2048 or higher
- **Validity**: Certificates are automatically renewed before expiration
- **Distribution**: Available through standard SAML metadata

### Troubleshooting SP Configuration

If your identity provider reports issues with SP information:

1. **Verify URLs**: Ensure all URLs use HTTPS and include the correct tenant ID
2. **Check Metadata**: Use the metadata URL to verify configuration
3. **Test Connectivity**: Ensure your IdP can reach FastComments endpoints
4. **Validate Format**: Confirm your IdP supports the SP information format

Common issues include:
- Incorrect tenant ID in URLs
- Network connectivity problems between IdP and FastComments
- IdP expecting different URL formats or additional configuration options