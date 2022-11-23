<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'

  interface Address {
    id: number
    street: string
    streetName: string
    buildingNumber: string
    city: string
    zipcode: string
    country: string
    county_code: string
    latitude: number
    longitude: number
  }

  interface Person {
    id: number
    firstname: string
    lastname: string
    email: string
    phone: string
    birthday: string
    gender: string
    website: string
    image: string
    address: Address
  }

  interface FakeResponse {
    status: string
    code: number
    total: number
    data: [Person]
  }
  let person: FakeResponse

  async function getPerson() {
    person = await invoke('get_person')
    console.log(person)
  }
</script>

<div>
  <button on:click={getPerson} class="m-2 bg-gray-200 p-2 rounded-md">Person</button>
  {#if person}
    <p class="m-2 font-thin text-sm">{person.data[0]?.firstname}</p>
  {/if}
</div>
