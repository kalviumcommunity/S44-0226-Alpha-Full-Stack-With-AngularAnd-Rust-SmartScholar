import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-application-form',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './application-form.html',
  styleUrl: './application-form.css',
})
export class ApplicationForm {
  form = {
    scholarship_name: '',
    category: '',
    date: '',
  };

  submit() {
    console.log('Form Data:', this.form);
  }
}
