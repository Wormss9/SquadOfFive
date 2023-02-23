<script>
	import { login } from '$lib/api/api';
	import { useForm, validators, HintGroup, Hint, email, required } from 'svelte-use-form';
	const form = useForm();
</script>

<form use:form on:submit|preventDefault={login}>
	<h1>Login</h1>

	<input type="name" name="name" use:validators={[required]} />
	<HintGroup for="name">
		<Hint on="required">This is a mandatory field</Hint>
		<Hint on="name" hideWhenRequired>Name is not valid</Hint>
	</HintGroup>

	<input type="password" name="password" use:validators={[required]} />
	<Hint for="password" on="required">This is a mandatory field</Hint>

	<input type="submit" value="Login" disabled={!$form.valid} />
</form>
<pre>
{JSON.stringify($form, null, ' ')}
</pre>

<style>
	:global(.touched:invalid) {
		border-color: red;
		outline-color: red;
	}
</style>
