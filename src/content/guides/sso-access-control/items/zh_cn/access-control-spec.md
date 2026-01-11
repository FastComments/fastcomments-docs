定义多个用户如何交互并进行测试是很复杂的。以下是我们用于访问控制的规范，您可以使用它来测试您的实现：

    页面 的 group ids 为 null，用户 的 group ids 为 null - 应该有访问权限。
    页面 的 group ids 为 null，用户 有 group ids - 应该有访问权限。
    页面 有 group ids，用户 的 group ids 为 null - 应该有访问权限。
    页面 有 group ids，用户 的 group ids 为 empty list - 不应该有访问权限。
    页面 有 group ids，用户 有 group ids - 存在交集 - 应该有访问权限。
    页面 有 group ids，用户 有 group ids - 不存在交集 - 不应该有访问权限。
    页面 的 group ids 是 empty list（无人有访问权限），用户 为 null - 不应该有访问权限。
    
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