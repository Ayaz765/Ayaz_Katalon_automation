import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://www.techlistic.com/p/selenium-practice-form.html')

WebUI.click(findTestObject('Object Repository/Tech-Repo/Page_/div_context httpschema.org,  type BlogPosti_d8cf22'))

WebUI.setText(findTestObject('Object Repository/Tech-Repo/Page_/input_First name_firstname'), 'Ddemo1')

WebUI.setText(findTestObject('Object Repository/Tech-Repo/Page_/input_Last name_lastname'), 'test')

WebUI.click(findTestObject('Object Repository/Tech-Repo/Page_/input_Gender_sex'))

WebUI.click(findTestObject('Object Repository/Tech-Repo/Page_/input_Years of Experience_exp'))

WebUI.setText(findTestObject('Object Repository/Tech-Repo/Page_/input_Date_datepicker'), '12/06/1996')

WebUI.click(findTestObject('Object Repository/Tech-Repo/Page_/input_Profession_profession'))

WebUI.click(findTestObject('Object Repository/Tech-Repo/Page_/input_Automation Tools_tool'))

WebUI.click(findTestObject('Object Repository/Tech-Repo/Page_/input_Automation Tools_tool_1'))

WebUI.closeBrowser()

