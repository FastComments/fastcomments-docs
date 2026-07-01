將 FastComments SDK 加入您的應用程式的 `build.gradle.kts` 檔案：

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

確保您的專案的 `settings.gradle.kts` 中已包含 Repsy 倉庫：

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // 其他倉庫...
    }
}
```