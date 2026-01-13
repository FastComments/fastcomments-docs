SAML (Security Assertion Markup Language) is an XML-based open standard for exchanging authentication and authorization data between parties, 
particularly between an identity provider (IdP) and a service provider (SP).

### SAML Nasıl Çalışır

SAML, kullanıcıların identity provider (IdP) ile bir kez kimlik doğrulaması yapmasına izin vererek single sign-on (SSO) sağlar ve böylece kimlik bilgilerini tekrar girmeden birden fazla uygulamaya sorunsuzca erişim sağlar. Bir kullanıcı FastComments'a erişmeye çalıştığında:

1. **Authentication Request**: FastComments kullanıcıyı identity provider'ınıza yönlendirir
2. **User Authentication**: Kullanıcı IdP'niz ile kimlik doğrulaması yapar (ör. Active Directory, Okta, Azure AD)
3. **SAML Response**: IdP, imzalı bir SAML assertion'ını FastComments'e gönderir
4. **User Access**: FastComments assertion'ı doğrular ve kimliği doğrulanmış kullanıcıya erişim izni verir

### SAML'in Faydaları

- **Enhanced Security**: Merkezi kimlik doğrulama, şifreyle ilgili güvenlik risklerini azaltır
- **Improved User Experience**: Kullanıcılar bir kez oturum açar ve birden fazla uygulamaya kesintisiz erişir
- **Compliance**: Erişim kontrolü ve denetim izleri için düzenleyici gereksinimlere uyulmasına yardımcı olur
- **Administrative Control**: BT yöneticileri merkezi kullanıcı yönetimini sürdürür

### SAML 2.0 Desteği

FastComments, SAML standardının en yaygın benimsenen sürümü olan SAML 2.0'ı uygular. Uygulamamız aşağıyı destekler:

- HTTP-POST ve HTTP-Redirect bindingleri
- İmzalı SAML yanıtları ve assertions
- Şifrelenmiş assertions (isteğe bağlı)
- Birden çok imza ve digest algoritması
- Çeşitli name identifier formatları