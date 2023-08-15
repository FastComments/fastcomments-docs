Sometimes FastComments has to email your users, especially if you are not using Secure SSO.

Examples of this includes verifying their account or activity when commenting for the first time. FastComments
will also send them notifications for replies to their comments.

When FastComments emails your users, we will use a default From Name and Email of `FastComments Robot` and `noreply@fastcomments.com`.

We'll also use our own logo in the footer of these emails.

If you have FastComments Flex or Pro, this all can be customized on a per-domain basis via the "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

When customizing the logo shown in emails, ensure that the size you are uploading is the same size that you want to show in the footer of the email.

### When Customizing The `From Domain`

If you customize the `From Domain`, Email providers and clients need to know that FastComments is authorized to send emails on your behalf. Otherwise,
defining the `From Domain` and not following the below steps likely will result in emails going to spam.

#### 1. Setup SPF

To allow FastComments to securely send email as your domain, ensure you add an SPF record that allows us to do so.

Ensure there are SPF records to allow `mail.fastcomments.com` and `sib.fastcomments.com` to send mail as your domain.

Some more information on how to do this is here: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Setup DKIM

In addition to SPF, you should set up DKIM. Once your DNS configuration is ready, [invoke the API](/guide-api.html#domain-config-structure) to define your DKIM configuration.

### Unsubscribe Links

When using SSO, the unsubscribe features used in emails and notifications can be customized [via the DomainConfigs API](/guide-api.html#domain-config-structure).
