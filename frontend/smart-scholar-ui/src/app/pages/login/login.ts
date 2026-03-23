import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router, RouterModule } from '@angular/router';
import { Auth } from '../../services/auth';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [FormsModule, RouterModule],
  templateUrl: './login.html',
  styleUrls: ['./login.css'],
})
export class Login {
  email: string = '';
  password: string = '';

  constructor(
    private authService: Auth,
    private router: Router,
  ) {}

  login() {
    if (!this.email || !this.password) {
      alert('Please fill all fields');
      return;
    }

    const demoToken = 'demo-token';

    // 👇 simulate roles (change for testing)
    let role = 5;

    this.authService.login(demoToken);
    localStorage.setItem('role', role.toString());

    this.router.navigate(['/']);
  }
}
