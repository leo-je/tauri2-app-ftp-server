export default {
  app: {
    title: 'FTP Server',
    name: 'FTP 服务器'
  },
  splash: {
    welcome: '欢迎使用 FTP Server',
    loading: '正在启动',
    ready: '准备就绪',
    init: {
      title: '系统初始化',
      systemCheck: '系统检测',
      systemCheckDesc: '检测操作系统和硬件环境',
      configLoad: '配置加载',
      configLoadDesc: '加载应用配置文件',
      serviceInit: '服务初始化',
      serviceInitDesc: '初始化 FTP 服务和网络接口',
      ready: '准备就绪',
      readyDesc: '系统初始化完成',
    },
    status: {
      pending: '等待中',
      running: '进行中',
      completed: '已完成',
      error: '错误',
    },
    error: {
      title: '初始化失败',
      retry: '重试',
      skip: '跳过',
      unknownError: '未知错误',
    },
    messages: {
      checkingSystem: '正在检测系统环境...',
      loadingConfig: '正在加载配置...',
      initService: '正在初始化服务...',
      finalizing: '正在完成初始化...',
      systemCheckComplete: '检测完成: {0} {1}',
      configLoaded: '配置加载成功',
      usingDefaultConfig: '使用默认配置',
      interfacesFound: '发现 {0} 个网络接口',
      systemReady: '系统准备就绪'
    }
  },
  status: {
    running: '运行中',
    stopped: '已停止',
    runtime: '运行时间'
  },
  config: {
    title: '服务器配置',
    shareDir: '共享目录',
    port: '服务端口',
    select: '选择',
    open: '打开',
    placeholder: {
      dir: '请选择共享目录',
      port: '21'
    }
  },
  connection: {
    title: '连接地址',
    copied: '已复制到剪贴板'
  },
  control: {
    start: '启动服务',
    stop: '停止服务',
    startTooltip: '启动服务',
    stopTooltip: '停止服务'
  },
  message: {
    selectPath: '请选择路径',
    noDir: '未选择目录',
    pathInvalid: '路径无效',
    portInvalid: '端口无效',
    serviceStarted: '服务已启动',
    serviceStopped: '服务已停止',
    serviceStartFailed: '服务启动失败',
    serviceStopFailed: '服务停止失败'
  },
  validation: {
    portWarning: {
      privilegedPort: '使用特权端口 {port} 可能需要管理员权限',
      ftpStandardPort: '使用标准 FTP 端口 21 可能需要管理员权限'
    }
  },
  language: {
    title: '语言',
    zh: '中文',
    en: 'English',
    ja: '日本語',
    th: 'ไทย'
  },
  tabs: {
    server: '服务器',
    auth: '权限设置',
    about: '关于'
  },
  auth: {
    serverRunning: '服务运行中，权限设置已锁定',
    anonymous: '匿名访问',
    anonymousDesc: '无需认证即可访问',
    readOnly: '只读',
    readWrite: '读写',
    userList: '用户列表',
    addUser: '添加用户',
    username: '用户名',
    password: '密码',
    actions: '操作',
    edit: '编辑',
    delete: '删除',
    noUsers: '暂无用户数据',
    addUserHint: '点击上方"添加用户"按钮创建新用户',
    editUser: '编辑用户',
    usernamePlaceholder: '请输入用户名',
    passwordPlaceholder: '请输入密码',
    permission: '权限',
    cancel: '取消',
    save: '保存',
    deleteConfirm: '确定要删除用户 "{username}" 吗？',
    deleteTitle: '删除确认',
    confirm: '确定',
    userDeleted: '用户已删除',
    userUpdated: '用户已更新',
    userAdded: '用户已添加',
    saveFailed: '保存数据失败'
  },
  about: {
    appName: 'FTP 服务器',
    version: '版本',
    clickToCopy: '点击复制版本号',
    versionCopied: '版本号已复制',
    developer: '开发者',
    developerName: 'FTP Server Team',
    description: '一款基于 Tauri 2.0 构建的轻量级、跨平台 FTP 服务器管理工具。提供现代化的用户界面，支持快速部署和安全文件传输服务。',
    techStack: '技术栈',
    features: '主要特性',
    featuresList: {
      crossPlatform: {
        title: '跨平台支持',
        desc: '支持 Windows、macOS 和 Linux 系统'
      },
      easyConfig: {
        title: '简单配置',
        desc: '直观的界面，快速启动 FTP 服务'
      },
      permissionControl: {
        title: '权限管理',
        desc: '灵活的用户权限和匿名访问控制'
      },
      realtime: {
        title: '实时监控',
        desc: '查看服务状态和运行时间'
      }
    },
    linksSection: '相关链接',
    links: {
      github: 'GitHub',
      documentation: '文档',
      feedback: '问题反馈',
      changelog: '更新日志'
    },
    copyright: '版权所有 © 2024 FTP Server Team',
    rights: '保留所有权利'
  },
  tray: {
    show: '显示主界面',
    start: '启动 FTP',
    stop: '停止 FTP',
    quit: '退出'
  }
}
