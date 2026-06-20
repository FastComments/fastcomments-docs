### Maven

將 Repsy 儲存庫新增到您專案的 POM：

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

接著新增您需要的相依項目：

```xml
<dependencies>
    <!-- API 用戶端 -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- 核心函式庫（包含 SSO） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub 函式庫（用於即時事件） -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

將 Repsy 儲存庫新增到您的 build.gradle 檔案：

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API 用戶端
    implementation "com.fastcomments:client:2.0.0"
    
    // 核心函式庫（包含 SSO）
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub 函式庫（用於即時事件）
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### 函式庫內容

此函式庫包含三個模組。已產生的 API 用戶端、包含手寫工具以便更容易使用 API 的核心 Java 函式庫，以及作為訂閱變更串流之用的 `pubsub` 模組。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### 公開 API 與受保護 API

對於 API 用戶端，有三個類別，`DefaultApi`、`PublicApi` 與 `ModerationApi`。`DefaultApi` 包含需要您 API 金鑰的方法，而 `PublicApi` 包含可以直接從瀏覽器／行動裝置等在未驗證情況下呼叫的方法。

`ModerationApi` 提供管理員儀表板的功能。它包含評論管理的方法（列出、計數、搜尋、日誌與匯出）、審核操作（移除／還原、檢舉、設定審查／垃圾郵件／核准狀態、投票，以及重新開啟／關閉討論串）、封鎖（封鎖評論、撤銷封鎖、預封鎖摘要、封鎖狀態與偏好設定，以及被封鎖使用者計數），以及徽章與信任（授予／移除徽章、手動徽章、取得／設定信任因子，以及使用者內部資料）。每個 `ModerationApi` 方法都接受一個 `sso` 參數，以便在代表已透過 SSO 驗證的管理員的情況下執行呼叫。