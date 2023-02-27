<template>
  <nav>
    <div v-if="$cookies.get('Authorization')">
      <router-link to="/rooms">Rooms</router-link>
    </div>
    <div v-else>
      <router-link to="/login">Login</router-link>
      <router-link to="/register">Register</router-link>
    </div>
  </nav>
  <router-view />
</template>
