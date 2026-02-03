FastComments 允許您要求首次留言者在提交留言前接受您的服務條款。

When enabled:
- **匿名使用者** 每次留言時都會看到服務條款核取方塊
- **已驗證使用者** 只會在其第一次留言，或當您更新您的服務條款時看到該核取方塊

### Configuration

前往小工具自訂頁面並啟用「Require Terms of Service acceptance」核取方塊。啟用後，您會看到以下選項：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode**：預設情況下，核取方塊會顯示「I agree to the Terms of Service and Privacy Policy」，並連結到兩份文件。選擇「Customize text per locale」以為每種語言提供自訂文字。
- **TOS Last Updated Date**：當您更新您的服務條款時，請設定此日期。於此日期之前接受過的使用者將需要重新接受。

### How It Works

- 服務條款接受時間戳記會以每位使用者及每則留言儲存
- 當使用者接受服務條款時，該日期會記錄在其使用者檔案中（每租戶）
- 如果您設定的「最後更新」日期晚於使用者接受的日期，他們將需要重新接受
- 對於無法被追蹤的匿名使用者，該核取方塊會在每次留言提交時出現