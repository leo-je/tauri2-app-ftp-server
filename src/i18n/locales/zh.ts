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
    portInvalid: '端口无效'
  },
  language: {
    title: '语言',
    zh: '中文',
    en: 'English'
  },
  tabs: {
    server: '服务器',
    auth: '权限设置'
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
  }
}
