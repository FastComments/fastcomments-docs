---
将 FastComments SDK 添加到你的应用的 `build.gradle.kts` 文件中：

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

确保在你的项目的 `settings.gradle.kts` 中拥有 Repsy 仓库：

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
---