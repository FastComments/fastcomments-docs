將 FastComments SDK 新增到您應用程式的 `build.gradle.kts` 檔案：

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

請確保您的專案的 `settings.gradle.kts` 中包含 Repsy 儲存庫：

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```