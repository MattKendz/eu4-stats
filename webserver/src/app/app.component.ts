import { animate, state, style, transition, trigger } from '@angular/animations';
import { AfterViewInit, Component, ViewChild, OnInit } from '@angular/core';
import { CommonModule, NgOptimizedImage } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatIconModule } from '@angular/material/icon'
import { MatPaginator, MatPaginatorModule } from '@angular/material/paginator';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { MatTabsModule } from '@angular/material/tabs';
import { Sort, MatSort, MatSortModule } from '@angular/material/sort';
import { BrowserModule, Title } from '@angular/platform-browser';
import { RouterModule, Routes } from '@angular/router';
import { NgChartsModule } from 'ng2-charts';
import { ChartOptions } from 'chart.js';
import * as stats from '../assets/parsed_country.json';

interface Eu4Stats {
  countries: CountryStats[];
}

interface CountryStats {
  tag: string;
  name: string;
  player: string | null;
  country: Country;
  military: Military;
  mana: Mana;
} 

interface Country {
  total_development: number;
  real_development: number;
  gp_score: number;
  powers_earned: number[];
  technology: number[];
  ideas: any[];
  total_ideas: number;
  current_manpower: number;
  max_manpower: number;
  average_monarch: number[];
  income: number;
  income_history: number[][];
  number_provinces: number;
  number_buildings: number;
  buildings_value: number;
  buildings_per_province: number;
  innovativeness: number;
  absolutism: number;
  average_development: number;
  average_development_real: number;
}

interface Military {
  army_tradition: number;
  army_morale: number;
  army_discipline: number;
  army_force_limit: number;
  army_professionalism: number;
  siege_ability: number;
  fort_defense: number;
  infantry_ability: number;
  cavalry_ability: number;
  artillery_ability: number;
  fire_dealt: number;
  fire_received: number;
  shock_dealt: number;
  shock_received: number;
  leader_fire: number;
  leader_shock: number;
  leader_maneuver: number;
  leader_siege: number;
  mercenary_discipline: number;
  naval_tradition: number;
  naval_morale: number;
  naval_force_limit: number;
}

interface Mana {
  mana_spent: number[];
  spent_developing: number[];
  developing_ratio: string;
  spent_tech: number;
  spent_culture: number;
  spent_coring: number;
  spent_inflation: number;
  spent_ideas: number;
  spent_force_march: number;
  spent_generals: number;
  spent_unjustified: number;
}

interface Dev {
  data: number[];
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
  animations: [
    trigger('detailExpand', [
      state('collapsed', style({height: '0px', minHeight: '0'})),
      state('expanded', style({height: '*'})),
      transition('expanded <=> collapsed', animate('225ms cubic-bezier(0.4, 0.0, 0.2, 1)')),
    ]),
    trigger('detailIncome', [
      state('collapsed', style({height: '0px', minHeight: '0'})),
      state('expanded', style({height: '*'})),
      transition('expanded <=> collapsed', animate('225ms cubic-bezier(0.4, 0.0, 0.2, 1)')),
    ]),
  ],
  imports: [CommonModule, NgOptimizedImage, FormsModule, RouterModule, MatCheckboxModule, MatIconModule, MatTabsModule, MatTableModule, MatSortModule, MatPaginatorModule, NgChartsModule],
  standalone: true,
})

export class AppComponent implements AfterViewInit {
  name = 'Angular';
  countryColumns: string[] = ['country_flag', 'country_name', 'total_dev', 'real_dev', 'gp_score', 'total_mana', 'tech', 'total_ideas', 'curr_manpower', 'max_manpower', 'avg_monarch', 'income', 'income_history', 'provinces', 'num_buildings', 'buildings_value', 'buildings_per_province', 'inno', 'absolutism', 'avg_dev', 'avg_dev_real', 'player'];
  militaryColumns: string[] = ['military_flag', 'country_name', 'army_tradition', 'army_morale', 'army_discipline', 'army_force_limit', 'army_professionalism', 'siege_ability', 'fort_defense', 'infantry_ability', 'cavalry_ability', 'artillery_ability', 'fire_dealt', 'shock_dealt', 'leader_fire', 'leader_shock', 'leader_maneuver', 'leader_siege', 'mercenary_discipline', 'naval_tradition', 'naval_morale', 'naval_force_limit', 'player'];
  manaColumns: string[] = ['mana_flag', 'country_name', 'mana_spent', 'icon', 'spent_developing', 'developing_ratio', 'spent_tech', 'spent_culture', 'spent_coring', 'spent_inflation', 'spent_ideas', 'spent_force_march', 'spent_generals', 'spent_unjustified', 'player'];
  
  filterCountry = {player: false};
  filteredCountries: CountryStats[] = [];
  countries: CountryStats[] = stats.countries;
  dataSource = new MatTableDataSource<CountryStats>(this.countries);
  expandedPie: boolean | null;
  expandedIncome: boolean | null;

  @ViewChild(MatSort) sort: MatSort;
  @ViewChild(MatPaginator) paginator: MatPaginator;

  public pieChartOptions: ChartOptions<'pie'> = {
    responsive: false,
  };
  public pieChartLabels = ['Admin', 'Diplo', 'Military'];
  public pieChartDatasets: Dev[] = [];

  public incomeChartOptions: ChartOptions<'line'> = {};
  public incomeChartDatasets: any[] = [];
  public incomeChartLabels: number[] = [];

  public constructor(private titleService: Title) {
    this.titleService.setTitle("EU4 Stats");
    this.sortCountries({active: 'total_dev', direction: 'desc'});
  }

  ngAfterViewInit() {  
    this.filteredCountries = this.countries.filter((x: any) => (
      x.player != '' || !this.filterCountry.player));
    this.dataSource.paginator = this.paginator;
    this.dataSource.sort = this.sort;
    this.expandedPie = null;
    this.expandedIncome = null;
  }

  filterCountryChange() {
    this.sortCountries(this.sort);
  }

  sortCountries(sort: Sort) {
    this.expandedPie = null;
    this.expandedIncome = null;
    const data = this.countries.slice();
    if (!sort.active || sort.direction === '') {
      this.countries = data;
      return;
    }

    data.sort((x, y) => {
      const isAsc = sort.direction === 'asc';
      const a = x.country;
      const b = y.country;
      switch (sort.active) {
        case 'country_name':
          return compare(x.name, y.name, isAsc);
        case 'total_dev':
          return compare(a.total_development, b.total_development, isAsc);
        case 'real_dev':
          return compare(a.real_development, b.real_development, isAsc);
        case 'gp_score':
          return compare(a.gp_score, b.gp_score, isAsc);
        case 'total_mana':
          return compareTotal(a.powers_earned, b.powers_earned, isAsc);
        case 'tech':
          return compareTech(a.technology, b.technology, isAsc);
        case 'total_ideas':
          return compare(a.total_ideas, b.total_ideas, isAsc);
        case 'curr_manpower':
          return compare(a.current_manpower, b.current_manpower, isAsc);
        case 'max_manpower':
          return compare(a.max_manpower, b.max_manpower, isAsc);
        case 'avg_monarch':
          return compareTotal(a.average_monarch, b.average_monarch, isAsc);
        case 'income':
          return compare(a.income, b.income, isAsc);
        case 'provinces':
          return compare(a.number_provinces, b.number_provinces, isAsc);
        case 'num_buildings':
          return compare(a.number_buildings, b.number_buildings, isAsc);
        case 'buildings_value':
          return compare(a.buildings_value, b.buildings_value, isAsc);
        case 'buildings_per_province':
          return compare(a.buildings_per_province, b.buildings_per_province, isAsc);
        case 'inno':
          return compare(a.innovativeness, b.innovativeness, isAsc);
        case 'absolutism':
          return compare(a.absolutism, b.absolutism, isAsc);
        case 'avg_dev':
          return compare(a.average_development, b.average_development, isAsc);
        case 'avg_dev_real':
          return compare(a.average_development_real, b.average_development_real, isAsc);
        case 'player':
          return compare(x.player, y.player, isAsc);
        default:
          return 0;
      }
    });
    this.filteredCountries = data.filter((x: any) => (
      x.player != null || !this.filterCountry.player));
    this.dataSource = new MatTableDataSource<CountryStats>(this.filteredCountries);
    this.dataSource.paginator = this.paginator;
  }

  sortMilitaries(sort: Sort) {
    this.expandedPie = null;
    this.expandedIncome = null;
    const data = this.countries.slice();
    if (!sort.active || sort.direction === '') {
      this.countries = data;
      return;
    }

    data.sort((x, y) => {
      const isAsc = sort.direction === 'asc';
      const a = x.military;
      const b = y.military;
      switch (sort.active) {
        case 'country_name':
          return compare(x.name, y.name, isAsc);
        case 'army_tradition':
          return compare(a.army_tradition, b.army_tradition, isAsc);
        case 'army_morale':
          return compare(a.army_morale, b.army_morale, isAsc);
        case 'army_discipline':
          return compare(a.army_discipline, b.army_discipline, isAsc);
        case 'army_force_limit':
          return compare(a.army_force_limit, b.army_force_limit, isAsc);
        case 'army_professionalism':
          return compare(a.army_professionalism, b.army_professionalism, isAsc);
        case 'siege_ability':
          return compare(a.siege_ability, b.siege_ability, isAsc);
        case 'fort_defense':
          return compare(a.fort_defense, b.fort_defense, isAsc);
        case 'infantry_ability':
          return compare(a.infantry_ability, b.infantry_ability, isAsc);
        case 'cavalry_ability':
          return compare(a.cavalry_ability, b.cavalry_ability, isAsc);
        case 'artillery_ability':
          return compare(a.artillery_ability, b.artillery_ability, isAsc);
        case 'fire_dealt':
          return compare(a.fire_dealt, b.fire_dealt, isAsc);
        case 'shock_dealt':
          return compare(a.shock_dealt, b.shock_dealt, isAsc);
        case 'leader_fire':
          return compare(a.leader_fire, b.leader_fire, isAsc);
        case 'leader_shock':
          return compare(a.leader_shock, b.leader_shock, isAsc);
        case 'leader_maneuver':
          return compare(a.leader_maneuver, b.leader_maneuver, isAsc);
        case 'leader_siege':
          return compare(a.leader_siege, b.leader_siege, isAsc);
        case 'mercenary_discipline':
          return compare(a.mercenary_discipline, b.mercenary_discipline, isAsc);
        case 'naval_tradition':
          return compare(a.naval_tradition, b.naval_tradition, isAsc);
        case 'naval_morale':
          return compare(a.naval_morale, b.naval_morale, isAsc);
        case 'naval_force_limit':
          return compare(a.naval_force_limit, b.naval_force_limit, isAsc);
        case 'player':
          return compare(x.player, y.player, isAsc);
        default:
          return 0;
      }
    });
    this.filteredCountries = data.filter((x: any) => (
      x.player != null || !this.filterCountry.player));
    this.dataSource = new MatTableDataSource<CountryStats>(this.filteredCountries);
    this.dataSource.paginator = this.paginator;
  }

  sortMana(sort: Sort) {
    this.expandedPie = null;
    this.expandedIncome = null;
    const data = this.countries.slice();
    if (!sort.active || sort.direction === '') {
      this.countries = data;
      return;
    }

    data.sort((x, y) => {
      const isAsc = sort.direction === 'asc';
      const a = x.mana;
      const b = y.mana;
      switch (sort.active) {
        case 'country_name':
          return compare(x.name, y.name, isAsc);
        case 'mana_spent':
          return compareTotal(a.mana_spent, b.mana_spent, isAsc);
        case 'spent_developing':
          return compareTotal(a.spent_developing, b.spent_developing, isAsc);
        case 'spent_tech':
          return compare(a.spent_tech, b.spent_tech, isAsc);
        case 'spent_culture':
          return compare(a.spent_culture, b.spent_culture, isAsc);
        case 'spent_coring':
          return compare(a.spent_coring, b.spent_coring, isAsc);
        case 'spent_inflation':
          return compare(a.spent_inflation, b.spent_inflation, isAsc);
        case 'spent_ideas':
          return compare(a.spent_ideas, b.spent_ideas, isAsc);
        case 'spent_force_march':
          return compare(a.spent_force_march, b.spent_force_march, isAsc);
        case 'spent_generals':
          return compare(a.spent_generals, b.spent_generals, isAsc);
        case 'spent_unjustified':
          return compare(a.spent_unjustified, b.spent_unjustified, isAsc);
        case 'player':
          return compare(x.player, y.player, isAsc);
        default:
          return 0;
      }
    });
    this.filteredCountries = data.filter((x: any) => (
      x.player != null || !this.filterCountry.player));
    this.dataSource = new MatTableDataSource<CountryStats>(this.filteredCountries);
    this.dataSource.paginator = this.paginator;
  }

  changePieData(c: CountryStats) {
    this.pieChartDatasets = [{data: c.mana.spent_developing}];
  }

  changeIncomeData(c: CountryStats) {
    let x: number[] = [];
    let y: number[] = [];

    c.country.income_history.forEach((i) => {
      x.push(i[0]);
      y.push(i[1] / 12);
    })
    this.incomeChartDatasets = [{
      label: 'Monthly Income',
      data: y,
      borderColor: 'rgb(0, 128, 0)',
      pointBackgroundColor: 'rgb(0, 128, 0)',
      pointBorderColor: 'rgb(0, 128, 0)',
      pointHitRadius: 5,
      pointRadius: 0,
    }];
    this.incomeChartLabels = x;
    this.incomeChartOptions = {
      responsive: false,
      scales: {
        x: {
          min: x[0],
          max: x[x.length - 1],
        },
        y: {
          beginAtZero: true,
        },
      },
    };
  }
}

function getSpentDev(countries: CountryStats[]){
  const dev: Dev[] = [];
  countries.forEach((c) => {
    dev.push({data: c.mana.spent_developing});
  })
  return dev;
}

function compare(a: number | string | null, b: number | string | null, isAsc: boolean) {
  if (a === null) {
    return isAsc ? 1 : -1;
  }
  if (b === null) {
    return isAsc ? -1 : 1;
  }
  return (a < b ? -1 : 1) * (isAsc ? 1 : -1);
}

function compareTotal(a: number[], b: number[], isAsc: boolean) {
  const aValue = a[0] + a[1] + a[2];
  const bValue = b[0] + b[1] + b[2];
  return (aValue < bValue ? -1 : 1) * (isAsc ? 1 : -1); 
}

function compareTech(a: number[], b: number[], isAsc: boolean) {
  const aValue = a[0] + a[1] + a[2];
  const bValue = b[0] + b[1] + b[2];
  if (aValue != bValue) {
    return (aValue < bValue ? -1 : 1) * (isAsc ? 1 : -1);
  }
  if (a[0] != b[0]) {
    return (a[0] < b[0] ? -1 : 1) * (isAsc ? 1 : -1);
  }
  if (a[1] != b[1]) {
    return (a[1] < b[1] ? -1 : 1) * (isAsc ? 1 : -1);
  }
  return (a[2] < b[2] ? -1 : 1) * (isAsc ? 1 : -1);
}