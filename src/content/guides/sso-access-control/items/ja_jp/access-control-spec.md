複数のユーザーがどのように相互作用するかを定義し、それをテストするのは複雑です。アクセス制御のために私たちが従う以下の仕様は、
実装をテストするために使用できます:

    Page with null group ids, user with null group ids - should have access.
    Page with null group ids, user with group ids - should have access.
    Page with group ids, user with null group ids - should have access.
    Page with group ids, user with empty list - should NOT have access.
    Page with group ids, user with group ids - intersection exists - should have access.
    Page with group ids, user with group ids - intersection does not exist - should NOT have access.
    Page with empty list of group ids (nobody has access), user with null - should NOT have access.
    
    SSO User A = group idsが定義されていない (null = フルアクセス).
    SSO User B = group idsが定義されていない (null = フルアクセス).
    Aは@Bできる.
    
    SSO User A = group idsが定義されていない (null = フルアクセス).
    SSO User B = Group ids defined.
    Aは@Bできる.
    
    SSO User A = Group ids defined.
    SSO User B = group idsが定義されていない (null = フルアクセス).
    Aは@Bできる.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    Aは@Bできない.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    Aは@Bできる.