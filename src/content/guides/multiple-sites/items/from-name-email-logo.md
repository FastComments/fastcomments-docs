Sometimes FastComments has to email your users, especially if you are not using Secure SSO.

Examples of this includes verifying their account or activity when commenting for the first time. FastComments
will also send them notifications for replies to their comments.

When FastComments emails your users, we will use a default From Name and Email of `FastComments Robot` and `noreply@fastcomments.com`.

We'll also use our own logo in the footer of these emails.

This all can be customized on a per-domain basis via the "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

When customizing the logo shown in emails, ensure that the size you are uploading is the same size that you want to show in the footer of the email.

### When Customizing The `From Domain`

To allow FastComments to securely send email as your domain, ensure you add an SPF record that allows us to do so.

Some more information on how to do this is here: https://mailtrap.io/blog/multiple-spf-records/

### When using SSO, the unsubscribe features used in emails and notifications can be customized [via the DomainConfigs API](/guide-api#domain-config-structure).
