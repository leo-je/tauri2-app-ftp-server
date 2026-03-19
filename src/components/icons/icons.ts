// SVG 图标路径定义
export const svgIcons = {
  // App 相关
  server: {
    paths: [
      'M12 2L2 7L12 12L22 7L12 2Z',
      'M2 17L12 22L22 17',
      'M2 12L12 17L22 12'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 窗口控制
  minimize: {
    paths: ['M5 12H19'],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round' }
  },
  close: {
    paths: ['M18 6L6 18M6 6L18 18'],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round' }
  },

  // 文件夹
  folder: {
    paths: ['M3 7V17C3 18.1046 3.89543 19 5 19H19C20.1046 19 21 18.1046 21 17V9C21 7.89543 20.1046 7 19 7H13L11 5H5C3.89543 5 3 5.89543 3 7Z'],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 锁/权限
  lock: {
    paths: [
      'M12 15V17M6 21H18C19.1046 21 20 20.1046 20 19V13C20 11.8954 19.1046 11 18 11H6C4.89543 11 4 11.8954 4 13V19C4 20.1046 4.89543 21 6 21ZM16 11V7C16 4.79086 14.2091 3 12 3C9.79086 3 8 4.79086 8 7V11H16Z'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 勾选圆形
  checkCircle: {
    paths: [
      'M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z',
      'M9 12L11 14L15 10'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 终端/代码
  terminal: {
    paths: [
      'M8 9L11 12L8 15M13 9L16 12L13 15',
      'M3 3H21V21H3V3Z'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round', rx: 2 }
  },

  // 目标/定位
  target: {
    paths: [
      'M12 2V6M12 18V22M2 12H6M18 12H22',
      'M12 12 m-3,0 a3,3 0 1,0 6,0 a3,3 0 1,0 -6,0'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round' }
  },

  // 播放/开始
  play: {
    paths: ['M8 5V19L19 12L8 5Z'],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 停止
  stop: {
    paths: ['M6 6H18M6 12H18M6 18H18'],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round' }
  },

  // 飞机/发送
  plane: {
    paths: ['M21 16V14L13 9V3.5C13 2.67 12.33 2 11.5 2S10 2.67 10 3.5V9L2 14V16L10 13.5V19L8 20.5V22L11.5 21L15 22V20.5L13 19V13.5L21 16Z'],
    attrs: { fill: 'currentColor' }
  },

  // 链接
  link: {
    paths: [
      'M13.8284 10.1716C12.2663 8.60948 9.73367 8.60948 8.17157 10.1716L6.75736 11.5858M17.2426 11.5858L18.6569 10.1716C20.219 8.60948 20.219 6.07683 18.6569 4.51473C17.0948 2.95263 14.5621 2.95263 13 4.51473L11.5858 5.92894M10.1716 13.8284C11.7337 15.3905 14.2663 15.3905 15.8284 13.8284L17.2426 12.4142M6.75736 12.4142L5.34315 13.8284C3.78105 15.3905 3.78105 17.9232 5.34315 19.4853C6.90524 21.0474 9.4379 21.0474 11 19.4853L12.4142 18.0711'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 复制
  copy: {
    paths: [
      'M8 5V19L19 12L8 5Z'
    ],
    attrs: { fill: 'currentColor' }
  },

  // 复制轮廓版
  copyOutline: {
    paths: [
      'M16 8V6C16 4.89543 15.1046 4 14 4H6C4.89543 4 4 4.89543 4 6V14C4 15.1046 4.89543 16 6 16H8',
      'M8 8H18C19.1046 8 20 8.89543 20 10V18C20 19.1046 19.1046 20 18 20H8C6.89543 20 6 19.1046 6 18V10C6 8.89543 6.89543 8 8 8Z'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2 }
  },

  // 时钟
  clock: {
    paths: [
      'M12 12 m-10,0 a10,10 0 1,0 20,0 a10,10 0 1,0 -20,0',
      'M12 6V12L16 14'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round' }
  },

  // 加号
  plus: {
    paths: [
      'M12 5V19M5 12H19'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round' }
  },

  // 用户组
  users: {
    paths: [
      'M17 21V19C17 16.7909 15.2091 15 13 15H5C2.79086 15 1 16.7909 1 19V21M23 21V19C22.9998 17.1771 22.265 15.4378 20.9551 14.1643C19.6452 12.8908 17.8668 12.1742 16.021 12.165M16 3C17.0609 3 18.0783 3.42143 18.8284 4.17157C19.5786 4.92172 20 5.93913 20 7C20 8.06087 19.5786 9.07828 18.8284 9.82843C18.0783 10.5786 17.0609 11 16 11C14.9391 11 13.9217 10.5786 13.1716 9.82843C12.4214 9.07828 12 8.06087 12 7C12 5.93913 12.4214 4.92172 13.1716 4.17157C13.9217 3.42143 14.9391 3 16 3Z'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 编辑
  edit: {
    paths: [
      'M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13M18.5 2.5C18.8978 2.10217 19.4374 1.87868 20 1.87868C20.5626 1.87868 21.1022 2.10217 21.5 2.5C21.8978 2.89782 22.1213 3.43739 22.1213 4C22.1213 4.56261 21.8978 5.10217 21.5 5.5L12 15L8 16L9 12L18.5 2.5Z'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 删除
  delete: {
    paths: [
      'M3 6H5H21M19 6V20C19 21.1046 18.1046 22 17 22H7C5.89543 22 5 21.1046 5 20V6M8 6V4C8 2.89543 8.89543 2 10 2H14C15.1046 2 16 2.89543 16 4V6M10 11V17M14 11V17'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 单个用户
  user: {
    paths: [
      'M20 21V19C20 16.7909 18.2091 15 16 15H8C5.79086 15 4 16.7909 4 19V21M16 7C16 9.20914 14.2091 11 12 11C9.79086 11 8 9.20914 8 7C8 4.79086 9.79086 3 12 3C14.2091 3 16 4.79086 16 7Z'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 全球/语言
  global: {
    paths: [
      'M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z',
      'M2 12H22',
      'M12 2C14.5013 4.73835 15.9228 8.29203 16 12C15.9228 15.708 14.5013 19.2616 12 22C9.49872 19.2616 8.07725 15.708 8 12C8.07725 8.29203 9.49872 4.73835 12 2Z'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 设置/齿轮
  settings: {
    paths: [
      'M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z',
      'M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1Z'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 下拉箭头
  arrowDown: {
    paths: [
      'M6 9L12 15L18 9'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 技术栈 Logos
  tauri: {
    paths: [
      'M13.912 0a8.72 8.72 0 0 0-8.308 6.139c1.05-.515 2.18-.845 3.342-.976 2.415-3.363 7.4-3.412 9.88-.097 2.48 3.315 1.025 8.084-2.883 9.45a6.131 6.131 0 0 1-.3 2.762 8.72 8.72 0 0 0 3.01-1.225A8.72 8.72 0 0 0 13.913 0zm.082 6.451a2.284 2.284 0 1 0-.15 4.566 2.284 2.284 0 0 0 .15-4.566zm-5.629.27a8.72 8.72 0 0 0-3.031 1.235 8.72 8.72 0 1 0 13.06 9.9131 10.173 10.174 0 0 1-3.343.965 6.125 6.125 0 1 1-7.028-9.343 6.114 6.115 0 0 1 .342-2.772zm1.713 6.27a2.284 2.284 0 0 0-2.284 2.283 2.284 2.284 0 0 0 2.284 2.284 2.284 2.284 0 0 0 2.284-2.284 2.284 2.284 0 0 0-2.284-2.284z'
    ],
    attrs: { fill: 'currentColor' }
  },

  vue: {
    paths: [
      'M24,1.61H14.06L12,5.16,9.94,1.61H0L12,22.39ZM12,14.08,5.16,2.23H9.59L12,6.41l2.41-4.18h4.43Z'
    ],
    attrs: { fill: 'currentColor' }
  },

  elementPlus: {
    paths: [
      'M12 2L2 7V17L12 22L22 17V7L12 2Z',
      'M12 12L2 7M12 12L22 7M12 12V22'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  typescript: {
    paths: [
      'M1.125 0C.502 0 0 .502 0 1.125v21.75C0 23.498.502 24 1.125 24h21.75c.623 0 1.125-.502 1.125-1.125V1.125C24 .502 23.498 0 22.875 0zm17.363 9.75c.612 0 1.154.037 1.627.111a6.38 6.38 0 0 1 1.306.34v2.458a3.95 3.95 0 0 0-.643-.361 5.093 5.093 0 0 0-.717-.26 5.453 5.453 0 0 0-1.426-.2c-.3 0-.573.028-.819.086a2.1 2.1 0 0 0-.623.242c-.17.104-.3.229-.393.374a.888.888 0 0 0-.14.49c0 .196.053.373.156.529.104.156.252.304.443.444s.423.276.696.41c.273.135.582.274.926.416.47.197.892.407 1.266.628.374.222.695.473.963.753.268.279.472.598.614.957.142.359.214.776.214 1.253 0 .657-.125 1.21-.373 1.656a3.033 3.033 0 0 1-1.012 1.085 4.38 4.38 0 0 1-1.487.596c-.566.12-1.163.18-1.79.18a9.916 9.916 0 0 1-1.84-.164 5.544 5.544 0 0 1-1.512-.493v-2.63a5.033 5.033 0 0 0 3.237 1.2c.333 0 .624-.03.872-.09.249-.06.456-.144.623-.25.166-.108.29-.234.373-.38a1.023 1.023 0 0 0-.074-1.089 2.12 2.12 0 0 0-.537-.5 5.597 5.597 0 0 0-.807-.444 27.72 27.72 0 0 0-1.007-.436c-.918-.383-1.602-.852-2.053-1.405-.45-.553-.676-1.222-.676-2.005 0-.614.123-1.141.369-1.582.246-.441.58-.804 1.004-1.089a4.494 4.494 0 0 1 1.47-.629 7.536 7.536 0 0 1 1.77-.201zm-15.113.188h9.563v2.166H9.506v9.646H6.789v-9.646H3.375z'
    ],
    attrs: { fill: 'currentColor' }
  },

  // 文件文本/日志
  fileText: {
    paths: [
      'M14 2H6C4.89543 2 4 2.89543 4 4V20C4 21.1046 4.89543 22 6 22H18C19.1046 22 20 21.1046 20 20V8L14 2Z',
      'M14 2V8H20',
      'M16 13H8',
      'M16 17H8',
      'M10 9H8'
    ],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 向上箭头
  chevronUp: {
    paths: ['M18 15L12 9L6 15'],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  },

  // 向下箭头
  chevronDown: {
    paths: ['M6 9L12 15L18 9'],
    attrs: { fill: 'none', stroke: 'currentColor', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'round' }
  }
} as const;

export type IconName = keyof typeof svgIcons;
