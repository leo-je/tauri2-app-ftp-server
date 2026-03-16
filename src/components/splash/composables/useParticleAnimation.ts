import { ref, onMounted, onUnmounted, Ref } from 'vue';

export interface ParticleOptions {
  maxParticles?: number;
  connectionDistance?: number;
  mouseInteractionDistance?: number;
  enableMouseInteraction?: boolean;
}

export interface UseParticleAnimationReturn {
  isRunning: Ref<boolean>;
  isReducedMotion: Ref<boolean>;
  start: () => void;
  stop: () => void;
}

// Particle 类
class Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  radius: number;
  color: string;

  constructor(canvasWidth: number, canvasHeight: number) {
    this.x = Math.random() * canvasWidth;
    this.y = Math.random() * canvasHeight;
    this.vx = (Math.random() - 0.5) * 0.5;
    this.vy = (Math.random() - 0.5) * 0.5;
    this.radius = Math.random() * 2 + 1;

    const colors = ['#667eea', '#764ba2', '#f093fb', '#4facfe', '#00f2fe'];
    this.color = colors[Math.floor(Math.random() * colors.length)];
  }

  update(canvasWidth: number, canvasHeight: number): void {
    this.x += this.vx;
    this.y += this.vy;

    if (this.x < 0 || this.x > canvasWidth) this.vx *= -1;
    if (this.y < 0 || this.y > canvasHeight) this.vy *= -1;

    // 限制速度
    const maxSpeed = 1;
    const speed = Math.sqrt(this.vx * this.vx + this.vy * this.vy);
    if (speed > maxSpeed) {
      this.vx = (this.vx / speed) * maxSpeed;
      this.vy = (this.vy / speed) * maxSpeed;
    }
  }

  draw(ctx: CanvasRenderingContext2D): void {
    // 绘制粒子
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
    ctx.fillStyle = this.color;
    ctx.fill();

    // 发光效果
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.radius * 3, 0, Math.PI * 2);
    const gradient = ctx.createRadialGradient(
      this.x, this.y, 0,
      this.x, this.y, this.radius * 3
    );
    gradient.addColorStop(0, this.color + '40');
    gradient.addColorStop(1, 'transparent');
    ctx.fillStyle = gradient;
    ctx.fill();
  }
}

// ParticleSystem 类
class ParticleSystem {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private particles: Particle[] = [];
  private animationId: number = 0;
  private options: ParticleOptions;
  private mouseX = 0;
  private mouseY = 0;
  private isActive = false;

  constructor(canvas: HTMLCanvasElement, options: ParticleOptions = {}) {
    this.canvas = canvas;
    const ctx = canvas.getContext('2d');
    if (!ctx) throw new Error('Could not get 2D context');
    this.ctx = ctx;
    this.options = {
      maxParticles: 80,
      connectionDistance: 120,
      mouseInteractionDistance: 150,
      enableMouseInteraction: true,
      ...options
    };

    this.resize();
    this.initParticles();
    this.bindEvents();
  }

  private resize(): void {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  private initParticles(): void {
    const { maxParticles = 80 } = this.options;
    const area = this.canvas.width * this.canvas.height;
    const particleCount = Math.min(maxParticles, Math.floor(area / 15000));

    this.particles = [];
    for (let i = 0; i < particleCount; i++) {
      this.particles.push(new Particle(this.canvas.width, this.canvas.height));
    }
  }

  private bindEvents(): void {
    window.addEventListener('resize', this.handleResize);

    if (this.options.enableMouseInteraction) {
      this.canvas.addEventListener('mousemove', this.handleMouseMove);
    }

    // 页面可见性变化时暂停/恢复
    document.addEventListener('visibilitychange', this.handleVisibilityChange);
  }

  private handleResize = (): void => {
    this.resize();
    this.initParticles();
  };

  private handleMouseMove = (e: MouseEvent): void => {
    const rect = this.canvas.getBoundingClientRect();
    this.mouseX = e.clientX - rect.left;
    this.mouseY = e.clientY - rect.top;
  };

  private handleVisibilityChange = (): void => {
    if (document.hidden) {
      this.pause();
    } else if (this.isActive) {
      this.resume();
    }
  };

  private drawConnections(): void {
    const { connectionDistance = 120 } = this.options;

    for (let i = 0; i < this.particles.length; i++) {
      for (let j = i + 1; j < this.particles.length; j++) {
        const p1 = this.particles[i];
        const p2 = this.particles[j];
        const dx = p1.x - p2.x;
        const dy = p1.y - p2.y;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance < connectionDistance) {
          const opacity = (1 - distance / connectionDistance) * 0.25;
          this.ctx.beginPath();
          this.ctx.strokeStyle = `rgba(102, 126, 234, ${opacity})`;
          this.ctx.lineWidth = 1;
          this.ctx.moveTo(p1.x, p1.y);
          this.ctx.lineTo(p2.x, p2.y);
          this.ctx.stroke();
        }
      }
    }
  }

  private applyMouseInteraction(): void {
    if (!this.options.enableMouseInteraction) return;

    const { mouseInteractionDistance = 150 } = this.options;

    for (const particle of this.particles) {
      const dx = this.mouseX - particle.x;
      const dy = this.mouseY - particle.y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      if (distance < mouseInteractionDistance && distance > 0) {
        const force = (mouseInteractionDistance - distance) / mouseInteractionDistance;
        particle.vx += (dx / distance) * force * 0.02;
        particle.vy += (dy / distance) * force * 0.02;
      }
    }
  }

  private animate = (): void => {
    if (!this.isActive) return;

    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

    // 更新和绘制粒子
    for (const particle of this.particles) {
      particle.update(this.canvas.width, this.canvas.height);
      particle.draw(this.ctx);
    }

    // 绘制连线
    this.drawConnections();

    // 鼠标交互
    this.applyMouseInteraction();

    this.animationId = requestAnimationFrame(this.animate);
  };

  start(): void {
    if (this.isActive) return;
    this.isActive = true;
    this.animate();
  }

  stop(): void {
    this.isActive = false;
    cancelAnimationFrame(this.animationId);
  }

  pause(): void {
    cancelAnimationFrame(this.animationId);
  }

  resume(): void {
    if (this.isActive) {
      this.animate();
    }
  }

  destroy(): void {
    this.stop();
    window.removeEventListener('resize', this.handleResize);
    this.canvas.removeEventListener('mousemove', this.handleMouseMove);
    document.removeEventListener('visibilitychange', this.handleVisibilityChange);
  }
}

// Composable
export function useParticleAnimation(
  canvasRef: Ref<HTMLCanvasElement | null>,
  options: ParticleOptions = {}
): UseParticleAnimationReturn {
  const isRunning = ref(false);
  const isReducedMotion = ref(false);
  let particleSystem: ParticleSystem | null = null;

  // 检测减少动画偏好
  const checkReducedMotion = (): void => {
    const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    isReducedMotion.value = mediaQuery.matches;
  };

  const start = (): void => {
    if (!canvasRef.value || isReducedMotion.value) return;

    particleSystem = new ParticleSystem(canvasRef.value, options);
    particleSystem.start();
    isRunning.value = true;
  };

  const stop = (): void => {
    particleSystem?.destroy();
    particleSystem = null;
    isRunning.value = false;
  };

  onMounted(() => {
    checkReducedMotion();

    // 监听系统偏好变化
    const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    mediaQuery.addEventListener('change', checkReducedMotion);
  });

  onUnmounted(() => {
    stop();
  });

  return {
    isRunning,
    isReducedMotion,
    start,
    stop
  };
}
