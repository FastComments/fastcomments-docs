預設情況下，FastComments 每天會將資料同步回您的 WordPress 網站。這純粹是備份用途，讓您持續擁有資料的一份副本，以及支援可能依賴該資料的外掛。

由於某些網站可處理大量讀取流量，但其資料庫部署並不總是能承受大量寫入流量（因此將這項工作卸載給 FastComments），因此並不會在每則評論儲存時立即同步。

可以透過安裝外掛自訂回寫到 WordPress 的同步排程。我們建議使用 [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description)。

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. 您可以設定該 hook 每小時、一天兩次、每日（預設），或每週執行一次。

The sync back to WordPress can also be performed any time manually by going to the FastComments plugin dashboard and selecting `Manually Sync`. 您將可以
選擇將資料同步回您的 WP 安裝，或將 WP 的評論重新上傳到 FastComments 伺服器。