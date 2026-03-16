# CSP（内容安全策略）配置说明

## 概述

内容安全策略（Content Security Policy，CSP）是一种安全标准，用于防止 XSS（跨站脚本攻击）、数据注入攻击等安全威胁。

## 当前配置

```json
{
  "security": {
    "csp": {
      "default-src": "'self'",
      "script-src": "'self' 'unsafe-inline' 'unsafe-eval'",
      "style-src": "'self' 'unsafe-inline'",
      "img-src": "'self' data: blob:",
      "font-src": "'self' data:",
      "connect-src": "'self' https://api.iconify.design",
      "object-src": "'none'",
      "base-uri": "'self'",
      "form-action": "'self'",
      "frame-ancestors": "'none'",
      "upgrade-insecure-requests": ""
    }
  }
}
```

## 配置详解

### 1. `default-src: 'self'`
**作用**：默认策略，仅允许从同源加载资源
**防护**：防止加载外部恶意资源
**适用**：所有未明确指定的资源类型

### 2. `script-src: 'self' 'unsafe-inline' 'unsafe-eval'`
**作用**：脚本加载策略
- `'self'`：允许同源脚本
- `'unsafe-inline'`：允许内联脚本（Vue 3 需要）
- `'unsafe-eval'`：允许 eval()（Vue 3 模板编译需要）

**为什么需要 `unsafe-*`**：
- Vue 3 在开发模式下使用内联脚本
- Element Plus 可能使用动态样式注入
- 生产环境可考虑移除（需测试）

**安全建议**：
```json
// 生产环境理想配置（需要测试）
"script-src": "'self'"
```

### 3. `style-src: 'self' 'unsafe-inline'`
**作用**：样式加载策略
- `'self'`：允许同源样式
- `'unsafe-inline'`：允许内联样式（Element Plus、动态主题需要）

**为什么需要 `unsafe-inline`**：
- Element Plus 动态样式注入
- Vue 组件的 scoped styles
- 动态主题切换功能

### 4. `img-src: 'self' data: blob:`
**作用**：图片加载策略
- `'self'`：允许同源图片
- `data:`：允许 base64 编码的图片
- `blob:`：允许 Blob URL 图片

**用途**：
- SVG 图标（可能使用 data URI）
- 动态生成的图片

### 5. `font-src: 'self' data:`
**作用**：字体加载策略
- `'self'`：允许同源字体
- `data:`：允许 base64 编码的字体

**用途**：
- Element Plus 图标字体
- 自定义字体

### 6. `connect-src: 'self' https://api.iconify.design`
**作用**：网络连接策略
- `'self'`：允许同源 AJAX/WebSocket
- `https://api.iconify.design`：允许 Iconify API（如果使用）

**注意**：如果应用不需要外部 API，可移除 `https://api.iconify.design`

### 7. `object-src: 'none'`
**作用**：禁止 `<object>`, `<embed>`, `<applet>` 标签
**防护**：防止嵌入恶意插件
**安全等级**：高

### 8. `base-uri: 'self'`
**作用**：限制 `<base>` 标签
**防护**：防止修改基础 URL 进行攻击

### 9. `form-action: 'self'`
**作用**：限制表单提交目标
**防护**：防止表单数据泄露到外部站点

### 10. `frame-ancestors: 'none'`
**作用**：禁止页面被嵌入到 iframe
**防护**：防止点击劫持攻击
**等同于**：`X-Frame-Options: DENY`

### 11. `upgrade-insecure-requests: ""`
**作用**：自动升级 HTTP 请求为 HTTPS
**防护**：防止混合内容攻击
**注意**：开发环境可能需要禁用

## 安全等级评估

| 指令 | 安全等级 | 说明 |
|------|---------|------|
| `default-src: 'self'` | ⭐⭐⭐⭐⭐ | 最高安全级别 |
| `script-src` | ⭐⭐⭐ | 因 Vue 3 需要 `unsafe-*` |
| `style-src` | ⭐⭐⭐⭐ | 因动态样式需要 `unsafe-inline` |
| `object-src: 'none'` | ⭐⭐⭐⭐⭐ | 完全禁用，最高安全 |
| `frame-ancestors: 'none'` | ⭐⭐⭐⭐⭐ | 完全防护点击劫持 |

**总体评分**：⭐⭐⭐⭐ (4/5)

## 开发环境 vs 生产环境

### 开发环境配置
```json
{
  "script-src": "'self' 'unsafe-inline' 'unsafe-eval'",
  "style-src": "'self' 'unsafe-inline'",
  "upgrade-insecure-requests": ""  // 可移除
}
```

### 生产环境配置（推荐）
```json
{
  "script-src": "'self'",  // 移除 unsafe-*（需测试）
  "style-src": "'self' 'unsafe-inline'",  // Element Plus 需要
  "upgrade-insecure-requests": ""
}
```

## 测试 CSP 配置

### 1. 使用 CSP 报告
添加报告端点：
```json
{
  "report-uri": "/csp-report",
  "report-to": "csp-endpoint"
}
```

### 2. 仅报告模式
```json
{
  "content-security-policy-report-only": "default-src 'self'; report-uri /csp-report"
}
```

### 3. 浏览器控制台检查
打开开发者工具控制台，查看 CSP 违规警告：
```
Refused to execute inline script because it violates the following Content Security Policy directive: "script-src 'self'"
```

## 常见问题排查

### 问题 1：Vue 3 应用白屏
**原因**：CSP 阻止了内联脚本
**解决**：添加 `'unsafe-inline' 'unsafe-eval'` 到 `script-src`

### 问题 2：样式不加载
**原因**：CSP 阻止了内联样式
**解决**：添加 `'unsafe-inline'` 到 `style-src`

### 问题 3：图标不显示
**原因**：CSP 阻止了字体或图片
**解决**：
- 添加 `data:` 到 `font-src`
- 添加 `data: blob:` 到 `img-src`

### 问题 4：网络请求失败
**原因**：CSP 阻止了外部连接
**解决**：添加域名到 `connect-src`

## Tauri 特定注意事项

### 1. 自定义协议
Tauri 使用自定义协议（如 `tauri://` 或 `https://tauri.localhost`），CSP 会自动适配。

### 2. IPC 通信
Tauri 的 IPC 通信不受 CSP 限制。

### 3. 本地资源
本地文件系统资源通过 `default-src: 'self'` 允许。

## 进一步加固建议

### 1. 移除 `unsafe-*`（高级）
如果可能，使用 nonce 或 hash 代替 `unsafe-inline`：

```html
<script nonce="random-nonce">
  // 内联脚本
</script>
```

```json
{
  "script-src": "'self' 'nonce-random-nonce'"
}
```

### 2. 使用 SRI（子资源完整性）
对外部资源添加完整性校验：
```html
<script src="https://example.com/library.js"
        integrity="sha384-..."
        crossorigin="anonymous"></script>
```

### 3. 限制内联样式
使用 CSS 文件代替内联样式，减少 `unsafe-inline` 的使用。

## CSP 验证工具

### 在线工具
- [CSP Evaluator](https://csp-evaluator.withgoogle.com/)
- [CSP Validator](https://validator.w3.org/)

### 浏览器扩展
- CSP Analyzer
- Content Security Policy Auditor

## 更新日志

**版本**: 0.1.1+
**日期**: 2026-03-04
**变更**:
- ✅ 启用 CSP 配置
- ✅ 配置适合 Vue 3 + Element Plus 的策略
- ✅ 添加全面的安全防护

## 参考资料

- [MDN: Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)
- [Tauri Security](https://tauri.app/v1/guides/security/)
- [Vue 3 CSP](https://vuejs.org/guide/best-practices/security.html#content-security-policy-csp)
- [OWASP CSP Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Content_Security_Policy_Cheat_Sheet.html)

---

**注意**：CSP 配置需要根据实际应用需求调整。建议在开发环境充分测试后再部署到生产环境。
