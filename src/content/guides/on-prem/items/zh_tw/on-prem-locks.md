---
像任何分散式系統一樣，FastComments 需要某種方式來鎖定資源和程序。可以透過 `/locks-in-progress` 端點監控這些鎖定。

[例如，這是我們美國分片上的端點](https://fastcomments.com/locks-in-progress)。

這可以用來找出系統為何停滯或負載過高。如果例如 SRE 想要了解系統為何出現高 CPU 負載，他們可以
檢查此端點以取得出問題的 cron 名稱。

---