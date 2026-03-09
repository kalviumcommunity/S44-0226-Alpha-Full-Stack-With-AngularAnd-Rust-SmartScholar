import { Routes } from '@angular/router';
import { Login } from './pages/login/login';
import { Register } from './pages/register/register';
import { MainLayout } from './layout/main-layout/main-layout';

export const routes: Routes = [
  { path: 'login', component: Login },
  { path: 'register', component: Register },

  {
    path: '',
    component: MainLayout,
    children: [{ path: '', redirectTo: 'dashboard', pathMatch: 'full' }],
  },
];
