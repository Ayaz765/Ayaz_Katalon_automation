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

WebUI.navigateToUrl('https://www.myntra.com/')

WebUI.setText(findTestObject('Object Repository/myntra/Page_Online Shopping for Women, Men, Kids F_27b4d1/input_Bag_desktop-searchBar'), 
    'men c')

WebUI.click(findTestObject('Object Repository/myntra/Page_Online Shopping for Women, Men, Kids F_27b4d1/li_Men Casual Shoes'))

WebUI.click(findTestObject('Object Repository/myntra/Page_Casual Shoes For Men - Buy Casual  Fla_af36c9/img__img-responsive'))

WebUI.switchToWindowTitle('Buy U.S. Polo Assn. Men White Clarkin Sneakers - Casual Shoes for Men 10339033 | Myntra')

WebUI.click(findTestObject('Object Repository/myntra/Page_Buy U.S. Polo Assn. Men White Clarkin _5f182c/p_8'))

WebUI.click(findTestObject('Object Repository/myntra/Page_Buy U.S. Polo Assn. Men White Clarkin _5f182c/div_ADD TO BAG'))

WebUI.click(findTestObject('Object Repository/myntra/Page_Buy U.S. Polo Assn. Men White Clarkin _5f182c/span_GO TO BAG_myntraweb-sprite pdp-whiteRi_5b975e'))

WebUI.click(findTestObject('Object Repository/myntra/Page_SHOPPING BAG/div_MOVE TO WISHLIST_itemComponents-base-in_6dd8b4'))

WebUI.closeBrowser()

