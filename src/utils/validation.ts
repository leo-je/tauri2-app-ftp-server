/**
 * 输入验证和清理工具
 *
 * 提供前端输入验证功能，与后端 Rust 验证逻辑保持一致
 */

/**
 * 验证路径
 * @param path - 待验证的路径
 * @returns 验证结果和错误消息
 */
export function validatePath(path: string): { valid: boolean; error?: string } {
  // 去除首尾空白
  const trimmed = path.trim();

  // 检查是否为空
  if (!trimmed) {
    return { valid: false, error: '路径不能为空' };
  }

  // 检查路径长度
  if (trimmed.length > 4096) {
    return { valid: false, error: '路径长度超出限制' };
  }

  // 检查危险模式
  const dangerousPatterns = ['../', '..\\\\', '\0', '<', '>', '|', '*', '?'];
  for (const pattern of dangerousPatterns) {
    if (trimmed.includes(pattern)) {
      return { valid: false, error: `路径包含非法字符或模式: ${pattern}` };
    }
  }

  return { valid: true };
}

/**
 * 验证端口号
 * @param port - 待验证的端口号
 * @returns 验证结果和错误消息
 */
export function validatePort(port: number | string): { valid: boolean; error?: string; warning?: string; warningKey?: string; warningParams?: Record<string, string> } {
  // 转换为数字
  const portNum = typeof port === 'string' ? parseInt(port.trim(), 10) : port;

  // 检查是否为有效数字
  if (isNaN(portNum)) {
    return { valid: false, error: '端口号必须是有效数字' };
  }

  // 检查端口范围
  if (portNum < 1 || portNum > 65535) {
    return { valid: false, error: '端口号必须在 1-65535 之间' };
  }

  // 警告信息
  const warnings: string[] = [];
  const warningKeys: string[] = [];
  const warningParams: Record<string, string>[] = [];

  if (portNum === 21) {
    warnings.push('使用标准 FTP 端口 21 可能需要管理员权限');
    warningKeys.push('validation.portWarning.ftpStandardPort');
    warningParams.push({});
  }
  if (portNum < 1024) {
    warnings.push(`使用特权端口 ${portNum} 可能需要管理员权限`);
    warningKeys.push('validation.portWarning.privilegedPort');
    warningParams.push({ port: portNum.toString() });
  }

  return {
    valid: true,
    warning: warnings.length > 0 ? warnings.join('；') : undefined,
    warningKey: warningKeys.length > 0 ? warningKeys[0] : undefined,
    warningParams: warningParams.length > 0 ? warningParams[0] : undefined
  };
}

/**
 * 验证用户名
 * @param username - 待验证的用户名
 * @returns 验证结果和错误消息
 */
export function validateUsername(username: string): { valid: boolean; error?: string } {
  const trimmed = username.trim();

  // 检查长度
  if (!trimmed) {
    return { valid: false, error: '用户名不能为空' };
  }
  if (trimmed.length > 64) {
    return { valid: false, error: '用户名长度不能超过 64 个字符' };
  }

  // 检查字符（允许字母、数字、下划线、短横线）
  const validPattern = /^[a-zA-Z0-9_-]+$/;
  if (!validPattern.test(trimmed)) {
    return { valid: false, error: '用户名只能包含字母、数字、下划线和短横线' };
  }

  return { valid: true };
}

/**
 * 验证密码
 * @param password - 待验证的密码
 * @returns 验证结果和错误消息
 */
export function validatePassword(password: string): { valid: boolean; error?: string; warning?: string } {
  // 检查长度
  if (!password) {
    return { valid: false, error: '密码不能为空' };
  }
  if (password.length > 128) {
    return { valid: false, error: '密码长度不能超过 128 个字符' };
  }

  // 检查密码强度（仅警告）
  const warnings: string[] = [];
  if (password.length < 6) {
    warnings.push('密码长度小于 6 位，建议使用更强的密码');
  }

  // 检查常见弱密码
  const weakPasswords = ['123456', 'password', 'admin', 'root', '12345678'];
  if (weakPasswords.includes(password)) {
    warnings.push('检测到弱密码，建议使用更强的密码');
  }

  return {
    valid: true,
    warning: warnings.length > 0 ? warnings.join('；') : undefined
  };
}

/**
 * 验证文件权限标识
 * @param fileAuth - 文件权限标识
 * @returns 验证结果和错误消息
 */
export function validateFileAuth(fileAuth: string): { valid: boolean; error?: string } {
  const trimmed = fileAuth.trim().toUpperCase();

  if (trimmed !== 'R' && trimmed !== 'W') {
    return { valid: false, error: "文件权限必须是 'R'（只读）或 'W'（读写）" };
  }

  return { valid: true };
}

/**
 * 验证用户数据
 * @param user - 用户对象
 * @returns 验证结果和错误消息
 */
export function validateUser(user: { username: string; password: string; fileAuth?: string }): {
  valid: boolean;
  errors: string[];
  warnings: string[];
} {
  const errors: string[] = [];
  const warnings: string[] = [];

  // 验证用户名
  const usernameResult = validateUsername(user.username);
  if (!usernameResult.valid) {
    errors.push(usernameResult.error!);
  }

  // 验证密码
  const passwordResult = validatePassword(user.password);
  if (!passwordResult.valid) {
    errors.push(passwordResult.error!);
  } else if (passwordResult.warning) {
    warnings.push(passwordResult.warning);
  }

  // 验证文件权限
  if (user.fileAuth) {
    const fileAuthResult = validateFileAuth(user.fileAuth);
    if (!fileAuthResult.valid) {
      errors.push(fileAuthResult.error!);
    }
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings
  };
}

/**
 * 验证用户列表
 * @param users - 用户列表
 * @returns 验证结果和错误消息
 */
export function validateUsers(users: Array<{ username: string; password: string; fileAuth?: string }>): {
  valid: boolean;
  errors: string[];
  warnings: string[];
} {
  const errors: string[] = [];
  const warnings: string[] = [];

  // 检查用户数量
  if (users.length > 100) {
    errors.push('用户数量不能超过 100');
  }

  // 验证每个用户
  for (let index = 0; index < users.length; index++) {
    const result = validateUser(users[index]);
    for (const err of result.errors) {
      errors.push(`用户 ${index + 1}: ${err}`);
    }
    for (const warn of result.warnings) {
      warnings.push(`用户 ${index + 1}: ${warn}`);
    }
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings
  };
}
