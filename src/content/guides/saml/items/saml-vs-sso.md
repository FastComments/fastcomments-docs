FastComments offers both SSO and SAML authentication. Understanding the differences helps you choose the right approach for your organization.

### Simple/Secure SSO Productions

FastComments offers two different SSO flows for authenticating into the comment widget through your site.
This is different from SAML, and does not require SAML. Instead, Simple SSO simply requires passing an object to
the comment widget, where Secure SSO does this plus hashing the payload with an API key.

SAML, on the other hand, authenticates the user to the entire product (based on their permissions) *as well as* the comment widget
(if they have third party cookies enabled for our domain).

### SAML Authentication

SAML is an enterprise-grade authentication protocol that provides more robust security and integration capabilities:

- **Implementation**: Requires Identity Provider (IdP) configuration and certificate exchange
- **Security**: Uses signed XML assertions and supports encryption
- **Use Case**: Ideal for enterprises with existing SAML infrastructure (Active Directory, Okta, etc.)
- **Setup Complexity**: More involved - requires IdP configuration and certificate management
- **Enterprise Features**: Advanced role mapping, centralized user management, audit trails

### When to Choose SAML

Consider SAML authentication if your organization:

- Already uses a SAML-compatible identity provider (Okta, Azure AD, ADFS, etc.)
- Requires enterprise-grade security and compliance
- Needs centralized user management and access control
- Has multiple applications using SAML for authentication
- Requires detailed audit trails and security reporting

### When to Choose Simple or Secure SSO

Our widget-focused SSO solutions might be sufficient if you:

- Have a custom authentication system
- Need quick implementation with minimal setup
- Don't require enterprise identity provider integration
- Want to control user data directly from your application
- Have simpler security requirements

Simple and Secure SSO are commonly used for online portals, blogs, etc, where the user already has an account *through your site or app*
but doesn't necessarily use SAML.
