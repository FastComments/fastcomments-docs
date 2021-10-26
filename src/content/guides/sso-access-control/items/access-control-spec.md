Defining how multiple users interact, and testing it, is complicated. Here is the following spec that we follow for access control,
which you may use to test your implementation:

    Page with null group ids, user with null group ids - should have access.
    Page with group ids, user with null group ids - should have access.
    Page with group ids, user with empty list - should NOT have access.
    Page with group ids, user with group ids - intersection exists - should have access.
    Page with group ids, user with group ids - intersection does not exist - should NOT have access.
    Page with empty list of group ids (nobody has access), user with null - should NOT have access.
    
    SSO User A = No group ids defined (null = full access).
    SSO User B = No group ids defined (null = full access).
    A can @B.
    
    SSO User A = No group ids defined (null = full access).
    SSO User B = Group ids defined.
    A can @B.
    
    SSO User A = Group ids defined.
    SSO User B = No group ids defined (null = full access).
    A can @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A can NOT @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A can @B.
