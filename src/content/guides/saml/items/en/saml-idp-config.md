After configuring SAML in FastComments, you need to set up FastComments as a Service Provider in your identity provider.

### General IdP Configuration

Most identity providers require the following information to add FastComments as a SAML application:

#### Required Service Provider Information

These values are automatically generated and displayed in your FastComments SAML configuration page:

**SP Entity ID / Audience**
- Format: `https://fastcomments.com/saml/{your-tenant-id}`
- This uniquely identifies your FastComments instance

**Assertion Consumer Service (ACS) URL**
- Format: `https://fastcomments.com/saml/callback/{your-tenant-id}`
- Where your IdP sends SAML responses after authentication

**SP Metadata URL** *(if supported by your IdP)*
- Format: `https://fastcomments.com/saml/metadata/{your-tenant-id}`
- Provides complete SAML configuration in XML format

**SAML Login URL**
- Format: `https://fastcomments.com/saml/login/{your-tenant-id}`
- Direct link to initiate SAML authentication

### Required SAML Attributes

Configure your identity provider to send these attributes with SAML responses:

#### Essential Attributes

**Email Address** *(Required)*
- **Attribute Name**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`
- **Purpose**: Unique user identification and notifications
- **Format**: Valid email address

#### Optional Attributes

**First Name**
- **Attribute Names**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`
- **Purpose**: User display name

**Last Name**
- **Attribute Names**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`
- **Purpose**: User display name

**Roles** *(Important for access control)*
- **Attribute Names**: `roles`, `groups`, `memberOf`, or custom attribute names
- **Purpose**: FastComments role assignment and permissions
- **Format**: Array of role strings or comma-separated values

### Common Identity Provider Configurations

#### Microsoft Azure AD

1. **Add Enterprise Application**
   - Search for "FastComments" or create a custom SAML application
   - Use the SP information provided by FastComments

2. **Configure Attributes**
   - Email: `user.mail` or `user.userprincipalname`
   - First Name: `user.givenname`
   - Last Name: `user.surname`
   - Roles: `user.assignedroles` or directory groups

#### Okta

1. **Create SAML Application**
   - Use "Create New App" and select SAML 2.0
   - Configure with FastComments SP information

2. **Attribute Statements**
   - Email: `user.email`
   - FirstName: `user.firstName`
   - LastName: `user.lastName`
   - Roles: `user.groups` or custom attributes

#### Google Workspace

1. **Add SAML Application**
   - Go to Apps > Web and mobile apps > Add App > Add custom SAML app
   - Configure with FastComments SP information

2. **Attribute Mapping**
   - Email: Primary email
   - First Name: First name
   - Last Name: Last name
   - Roles: Groups or custom attributes

#### Active Directory Federation Services (ADFS)

1. **Add Relying Party Trust**
   - Use the FastComments metadata URL or manual configuration
   - Configure SP information as provided

2. **Claim Rules**
   - Email: Email Address claim
   - Name: Name ID claim
   - Roles: Group membership or custom claims

### Attribute Name Flexibility

FastComments accepts role information from multiple attribute names to accommodate different IdP configurations:

- `roles`
- `groups`
- `memberOf`
- `role`
- `group`
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

This flexibility ensures compatibility with various identity providers without requiring specific attribute naming conventions.

### Testing Your Configuration

After configuring your identity provider:

1. Save the IdP configuration
2. Test with a dedicated test user account
3. Verify that attributes are being sent correctly
4. Check that roles are properly mapped
5. Ensure the authentication flow completes successfully

Most identity providers offer SAML testing tools to validate the configuration before deploying to production users.