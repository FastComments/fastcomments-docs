Najveće razlike između Disqus i FastComments Secure SSO su u tome što Disqus koristi SHA1 za enkripciju, dok mi koristimo SHA256.

To znači da je migracija sa Disqus laka - promenite algoritam heširanja sa SHA1 na SHA256 i ažurirajte nazive svojstava koji se prosleđuju UI.