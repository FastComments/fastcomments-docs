將 FastComments SDK 加入到您應用程式的 `build.gradle.kts` 檔案中：

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

請確保您專案的 `settings.gradle.kts` 中包含 Repsy 儲存庫：

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