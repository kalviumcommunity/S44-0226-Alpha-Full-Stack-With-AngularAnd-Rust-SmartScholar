import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-application-form',
  standalone: true,
  imports: [FormsModule, CommonModule],
  templateUrl: './application-form.html',
  styleUrl: './application-form.css',
})
export class ApplicationForm {
  form = {
    scholarship_name: '',
    category: '',
    date: '',
  };

  openDatePicker(event: any) {
    const input = event.target as HTMLInputElement;

    if (input.showPicker) {
      input.showPicker(); // ✅ opens picker on full click
    }
  }

  submit(form: any) {
    if (form.invalid) {
      return;
    }

    console.log('Valid Form:', this.form);
  }
}
