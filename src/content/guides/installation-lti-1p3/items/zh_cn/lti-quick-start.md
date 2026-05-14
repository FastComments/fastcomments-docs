---
1. 登录 FastComments 并转到 <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">您的 LTI 1.3 配置页面</a>。
2. (Optional) 从 **Platform** 下拉菜单中选择您正在连接的平台 - 它会设置显示标签，但 Auto-detect 工作正常。
3. 点击 **Generate URL**。会出现一个一次性 **Registration URL**（有效期 30 分钟，仅可使用一次）。
4. 在您的 LMS 中，打开 LTI 1.3 Dynamic Registration 界面，并将该 URL 粘贴到 **Tool initiation registration endpoint**（或相应）字段。提交。
5. 您的 LMS 会回调到 FastComments，交换密钥并创建集成。完成后弹出窗口会自动关闭。
6. 回到 FastComments，新的配置会出现在 **Existing Configurations** 表中。该工具现在可以在您的 LMS 课程中使用。
---