import { Routes } from '@angular/router';

import { Login } from './pages/login/login';
import { Register } from './pages/register/register';
import { MainLayout } from './layout/main-layout/main-layout';
import { authGuard } from './guards/auth-guard';
import { ApplicationForm } from './pages/application-form/application-form';

export const routes: Routes = [
  { path: 'login', component: Login },
  { path: 'register', component: Register },

  {
    path: '',
    component: MainLayout,
    canActivate: [authGuard],
    children: [
      { path: 'applications', component: ApplicationForm },
      { path: '', redirectTo: 'home', pathMatch: 'full' },
    ],
  },

  { path: '**', redirectTo: 'login' },
];
